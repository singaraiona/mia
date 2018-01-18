use std::fmt;
use std::mem;
use std::cell::UnsafeCell;
use function;
use special;
use stack::Stack;
use eval;
use context::*;

pub const FMT_ITEMS_LIMIT: usize = 30;

#[derive(Debug)]
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "** Error: `{}`", self.0)
    }
}

// Since there is no (yet?) gcc's like __function__ macro
macro_rules! func {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            extern crate core;
            unsafe { core::intrinsics::type_name::<T>() }
        }
        let name = type_name_of(f);
        let nm = &name[6..name.len() - 4];
        let cut = nm.find("::{").unwrap_or(nm.len());
        format!("{}:", &nm[..cut])
    }}
}

macro_rules! error_fmt {
    ($x:expr)                => { format!("{}", $x)                         };
    ($x:expr, $($y:expr),+)  => { format!("{} {}", $x, error_fmt!($($y),+)) }
}

macro_rules! eval_error { ($($x:expr),+) => { $crate::mia::Error(error_fmt!($($x),+)) }}
macro_rules! eval_err   { ($($x:expr),+) => { Err(eval_error!($($x),+))               }}

// Common errors
macro_rules! nyi_error   { ()                 => { eval_error!(func!(), "nyi.")                                     } }
macro_rules! args_error  { ($a:expr)          => { eval_error!(func!(), "invalid args:", format_list!($a))          } }
macro_rules! call_error  { ($a:expr)          => { eval_error!(func!(), "call:", $a, "is not callable.")            } }
macro_rules! undef_error { ($a:expr)          => { eval_error!(func!(), "undefined symbol:", $a)                    } }
macro_rules! bound_error { ($($a:expr),+)     => { eval_error!(func!(), "index out of bounds:", $($a),+)            } }
macro_rules! io_error    { ($($a:expr),+)     => { eval_error!(func!(), "I/O:", $($a),+)                            } }
macro_rules! arity_error { ($x:expr, $y:expr) => { eval_error!(func!(), "expected", $x, "arguments,", $y, "passed.") } }

macro_rules! nyi_err     { ()             => { Err(nyi_error!())                                                    } }
macro_rules! args_err    { ($a:expr)      => { Err(args_error!($a))                                                 } }
macro_rules! call_err    { ($a:expr)      => { Err(call_error!($a))                                                 } }
macro_rules! undef_err   { ($a:expr)      => { Err(undef_error!($a))                                                } }
macro_rules! bound_err   { ($($a:expr),+) => { Err(bound_error!($($a),+))                                           } }
macro_rules! io_err      { ($($a:expr),+) => { Err(io_error!($($a),+))                                              } }
macro_rules! arity_err   { ($x:expr, $y:expr) => { Err(arity_error!($x, $y))                                        } }

// MIA's datatypes
macro_rules! long     { ($v:expr)          => { AST::Long($v)                                       } }
macro_rules! float    { ($v:expr)          => { AST::Float($v)                                      } }
macro_rules! symbol   { ($v:expr)          => { AST::Symbol($v)                                     } }
macro_rules! sym      { ($v:expr)          => { AST::Symbol(new_symbol($v.to_string()))             } }
macro_rules! NIL      { ()                 => { AST::Symbol(0)                                      } }
macro_rules! T        { ()                 => { AST::Symbol(1)                                      } }
macro_rules! STRING   { ($v:expr)          => { AST::String(Box::new($v))                           } }
macro_rules! FUNCTION { ($v:expr)          => { AST::Function($v)                                   } }
macro_rules! LAMBDA   { ($a:expr, $b:expr) => { AST::Lambda(Box::new(Lambda { args:$a, body: $b })) } }
macro_rules! SPECIAL  { ($v:expr)          => { AST::Special($v)                                    } }
macro_rules! LONG     { ($v:expr)          => { AST::Vlong(Box::new($v))                            } }
macro_rules! FLOAT    { ($v:expr)          => { AST::Vfloat(Box::new($v))                           } }
macro_rules! LIST     { ($v:expr)          => { AST::List(Box::new($v))                             } }

pub type Value    = Result<AST, Error>;
pub type Vvalue   = Result<Vec<AST>, Error>;
// Evaluates all arguments before call
pub type Function = fn(&[AST], &mut Context) -> Value;
// It's up to calee to decide if arguments need evaluation
pub type Special  = fn(&[AST], &mut Context) -> Value;

lazy_static! {
    static ref _FUNCTIONS: [(&'static str, Function);9] =
        [("+",      function::plus), ("-",     function::minus),
         ("til",     function::til), ("=",     function::equal),
         ("eval",  eval::fold_list), ("prin",   function::prin),
         ("prinl", function::prinl), ("pp",       function::pp),
         ("load",   function::load)];

    static ref _SPECIALS: [(&'static str, Special);8] =
        [("setq",   special::setq), ("de",           special::de),
         ("quote", special::quote), ("'",         special::quote),
         ("time",   special::time), ("for",     special::forcond),
         ("if",   special::ifcond), ("while", special::whilecond)];
}

pub fn build_symbol(sym: &str) -> AST {
    //for f in _FUNCTIONS.iter() { if f.0 == sym { return AST::Function(f.1) } }
    //for s in _SPECIALS.iter()  { if s.0 == sym { return AST::Special(s.1) } }
    //symbol!(new_symbol(sym.to_string()))
    NIL!()
}

pub fn quoted(a: AST) -> AST { LIST!(vec![SPECIAL!(special::quote), a]) }

#[derive(Clone)]
pub struct Lambda {
    pub args: Vec<AST>,
    pub body: Vec<AST>,
}

#[derive(Clone)]
pub enum AST {
    Long(i64),
    Float(f64),
    Symbol(usize),
    String(Box<String>),
    Function(Function),
    Lambda(Box<Lambda>),
    Special(Special),
    Vlong(Box<Vec<i64>>),
    Vfloat(Box<Vec<f64>>),
    List(Box<Vec<AST>>),
}

impl PartialEq for AST {
    fn eq(&self, other: &AST) -> bool {
        if mem::discriminant(self) != mem::discriminant(other) { return false; }
        match (self, other) {
            (&AST::Long(l),       &AST::Long(r))       => l == r,
            (&AST::Float(l),      &AST::Float(r))      => l == r,
            (&AST::Symbol(l),     &AST::Symbol(r))     => l == r,
            (&AST::Function(l),   &AST::Function(r))   => l as i64 == r as i64,
            (&AST::Special(l),    &AST::Special(r))    => l as i64 == r as i64,
            (&AST::Vlong(ref l),  &AST::Vlong(ref r))  => l == r,
            (&AST::Vfloat(ref l), &AST::Vfloat(ref r)) => l == r,
            (&AST::List(ref l),   &AST::List(ref r))   => l.iter().zip(r.iter()).all(|(l, r)| l == r),
            _                                          => false,
        }
    }
}

// Avoid match destructuring when we exactly know the type
macro_rules! unwrap {
    ($ast:expr, $t:tt, $r:ty) => {
        match *$ast {
            AST::$t(ref x) => x,
            _ => unreachable!(),
        }
    }
}

impl AST {
    pub fn long(&self)   -> i64       { *unwrap!(self, Long, i64)                     }
    pub fn symbol(&self) -> usize     { *unwrap!(self, Symbol, usize)                 }
    pub fn list(&self)   -> &Vec<AST> {  unwrap!(self, List, &Box<Vec<AST>>).as_ref() }
    pub fn string(&self) -> &str      {  unwrap!(self, String, String).as_str()       }
    pub fn is_nil(&self) -> bool      {
        match *self {
            AST::Symbol(s) => s == 0,
            AST::List(ref l) => l.is_empty(),
            _ => false,
        }
    }
    pub fn to_string(&self) -> String {
        if let AST::String(ref s) = *self { return format!("{}", s); }
        format!("{}", self)
    }
}

macro_rules! format_seq { ($l:expr) => {
    {
        let _suf = if $l.len() < FMT_ITEMS_LIMIT { "" } else { ".." };
        format!("{}{}", $l.iter().take(FMT_ITEMS_LIMIT).map(|v| format!("{}", v))
                        .collect::<Vec<_>>().join(" "), _suf)
    }
}}

macro_rules! format_list { ($l:expr) => { format!("({})", format_seq!($l)) } }

macro_rules! format_builtin {
    ($p:expr,$s:expr) => {
        $p.iter().map(|x| (x.0, x.1 as i64))
        .find(|&x| x.1 == $s as i64).map(|x| x.0.to_string())
        .unwrap_or(format!("Builtin: {} can't be formatted.", $s as i64))
    }
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AST::Long(x)       => write!(f, "{}", x),
            AST::Float(x)      => write!(f, "{}", x),
            AST::String(ref x) => write!(f, "\"{}\"", x),
            AST::Symbol(x)     => write!(f, "{}", symbol_to_str(x)),
            AST::Function(x)   => write!(f, "{}", format_builtin!(_FUNCTIONS, x)),
            AST::Lambda(ref x) => write!(f, "({} {})", format_list!(x.args), format_seq!(x.body)),
            AST::Special(x)    => write!(f, "{}", format_builtin!(_SPECIALS, x)),
            AST::List(ref x)   => write!(f, "{}", format_list!(x)),
            AST::Vlong(ref x)  => write!(f, "#l{}", format_list!(x)),
            AST::Vfloat(ref x) => write!(f, "#f{}", format_list!(x)),
        }
    }
}

