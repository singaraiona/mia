use std::fmt;
use std::mem;
use std::cell::UnsafeCell;
use function;
use special;
use stack::Stack;
use eval;

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
        format!("{}:", &name[6..name.len() - 4])
    }}
}

macro_rules! error_fmt {
    ($x:expr)                => { format!("{}", $x)                         };
    ($x:expr, $($y:expr),+)  => { format!("{} {}", $x, error_fmt!($($y),+)) }
}

macro_rules! eval_error { ($($x:expr),+) => { $crate::mia::Error(error_fmt!($($x),+)) }}
macro_rules! eval_err   { ($($x:expr),+) => { Err(eval_error!($($x),+))               }}

// Common errors
macro_rules! nyi_err   { ()             => { eval_err!(func!(), "nyi.")                            } }
macro_rules! args_err  { ($a:expr)      => { eval_err!(func!(), "invalid args:", format_list!($a)) } }
macro_rules! bound_err { ($($a:expr),+) => { eval_err!(func!(), "index out of bounds:", $($a),+)   } }

// MIA's datatypes
macro_rules! long     { ($v:expr)          => { AST::Long($v)                                       } }
macro_rules! float    { ($v:expr)          => { AST::Float($v)                                      } }
macro_rules! symbol   { ($v:expr)          => { AST::Symbol($v)                                     } }
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
pub type Function = fn(&[AST]) -> Value;
// It's up to calee to decide if arguments need evaluation
pub type Special  = fn(&[AST]) -> Value;

lazy_static! {
    static ref _FUNCTIONS: [(&'static str, Function);5] =
        [("+",     function::plus), ("-",     function::minus),
         ("til",    function::til), ("=",     function::equal),
         ("eval", eval::fold_list)];

    static ref _SPECIALS: [(&'static str, Special);7] =
        [("setq",   special::setq), ("de",       special::de),
         ("quote", special::quote), ("'",     special::quote),
         ("time",   special::time), ("each",   special::each),
         ("if",   special::ifcond)];
}

pub fn build_symbol(sym: &str) -> AST {
    for f in _FUNCTIONS.iter() { if f.0 == sym { return AST::Function(f.1) } }
    for s in _SPECIALS.iter()  { if s.0 == sym { return AST::Special(s.1) } }
    symbol!(new_symbol(sym.to_string()))
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
            (&AST::Long(l), &AST::Long(r)) => l == r,
            (&AST::Float(l), &AST::Float(r)) => l == r,
            (&AST::Symbol(l), &AST::Symbol(r)) => l == r,
            (&AST::Function(l), &AST::Function(r)) => l as i64 == r as i64,
            (&AST::Special(l), &AST::Special(r)) => l as i64 == r as i64,
            (&AST::Vlong(ref l), &AST::Vlong(ref r)) => l == r,
            (&AST::Vfloat(ref l), &AST::Vfloat(ref r)) => l == r,
            (&AST::List(ref l), &AST::List(ref r)) => l.iter().zip(r.iter()).all(|(l, r)| l == r),
            _ => false,
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
    pub fn long(&self) -> i64 { *unwrap!(self, Long, i64) }
    pub fn symbol(&self) -> usize { *unwrap!(self, Symbol, usize) }
    pub fn list(&self) -> &Vec<AST> { unwrap!(self, List, &Box<Vec<AST>>).as_ref() }
    pub fn string(&self) -> &str { unwrap!(self, String, String).as_str() }
    pub fn is_nil(&self) -> bool {
        match *self {
            AST::Symbol(s) => s == 0,
            AST::List(ref l) => l.is_empty(),
            _ => false,
        }
    }
}

macro_rules! format_seq { ($l:expr) => {
    {
        let _suf = if $l.len() < FMT_ITEMS_LIMIT { "" } else { "..." };
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

thread_local! {
    pub static _SYMBOLS: UnsafeCell<Vec<String>> = UnsafeCell::new(Vec::new());
    pub static _STACK:   UnsafeCell<Stack>       = UnsafeCell::new(Stack::new());
}

pub fn new_symbol(sym: String) -> usize {
    unsafe {
        _SYMBOLS.with(|s| {
            let syms = &mut (*s.get());
            for (i, x) in syms.iter().enumerate() { if *x == sym { return i; } }
            syms.push(sym);
            syms.len() - 1
        })
    }
}

pub fn symbol_to_str(sym: usize) -> &'static str { unsafe { _SYMBOLS.with(|s| &(*s.get())[sym]) } }

pub fn insert_entry(sym: usize, ast: AST) { unsafe { _STACK.with(|s| { (*s.get()).insert(sym, ast) }); } }

pub fn entry(sym: usize) -> Value {
    unsafe {
        _STACK.with(|s| {
            (*s.get()).entry(sym).ok_or_else(|| eval_error!("Undefined symbol:", symbol_to_str(sym)))
        })
    }
}

fn init_builtin_symbol(sym: &str, ast: AST) {
    let id = new_symbol(sym.to_string());
    insert_entry(id, ast);
}

pub fn push_frame() { unsafe { _STACK.with(|s| (*s.get()).push_frame()) } }

pub fn pop_frame() { unsafe { _STACK.with(|s| (*s.get()).pop_frame()) } }

pub fn init_builtin_symbols() {
    init_builtin_symbol("NIL",  NIL!());
    init_builtin_symbol("T",    T!());
}

