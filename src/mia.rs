use std::fmt;
use std::cell::UnsafeCell;
use fnv::FnvHashMap;
use function;
use special;

#[derive(Debug)]
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "** error `{}`", self.0)
    }
}

#[macro_export]
macro_rules! error_fmt {
    ($x:expr)                => { format!("{}", $x)                         };
    ($x:expr, $($y:expr),+)  => { format!("{} {}", $x, error_fmt!($($y),+)) }
}

#[macro_export]
macro_rules! eval_error { ($($x:expr),+) => { $crate::mia::Error(error_fmt!($($x),+)) }}
#[macro_export]
macro_rules! eval_err   { ($($x:expr),+) => { Err(eval_error!($($x),+)) }}

// MIA's datatypes
macro_rules! long     { ($v:expr) => { AST::Long($v)              } }
macro_rules! float    { ($v:expr) => { AST::Float($v)             } }
macro_rules! symbol   { ($v:expr) => { AST::Symbol($v)            } }
macro_rules! NIL      { ()        => { AST::Symbol(0)             } }
macro_rules! T        { ()        => { AST::Symbol(1)             } }
macro_rules! STRING   { ($v:expr) => { AST::String(Box::new($v))  } }
macro_rules! FUNCTION { ($v:expr) => { AST::Function($v)          } }
macro_rules! SPECIAL  { ($v:expr) => { AST::Special($v)           } }
macro_rules! LONG     { ($v:expr) => { AST::Vlong(Box::new($v))   } }
macro_rules! LIST     { ($v:expr) => { AST::List(Box::new($v))    } }

pub type Value    = Result<AST, Error>;
pub type Vvalue   = Result<Vec<AST>, Error>;
// Evaluates all arguments before call
pub type Function = fn(&[AST]) -> Value;
// It's up to calee to decide if arguments need evaluation
pub type Special  = fn(&[AST]) -> Value;

#[derive(Clone)]
pub enum AST {
    Long(i64),
    Float(f64),
    Symbol(usize),
    String(Box<String>),
    Function(Function),
    Special(Special),
    Vlong(Box<Vec<i64>>),
    List(Box<Vec<AST>>),
}

macro_rules! format_list { ($l:expr) => { $l.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(" ") } }

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AST::Long(ref x)     => write!(f, "{}", x),
            AST::Float(ref x)    => write!(f, "{}", x),
            AST::String(ref x)   => write!(f, "\"{}\"", x),
            AST::Symbol(x)       => write!(f, "{}", symbol_to_str(x)),
            AST::Function(ref x) => write!(f, "{:#p}", x),
            AST::Special(ref x)  => write!(f, "{:?}", *x as i64),
            AST::List(ref x)     => write!(f, "({})", format_list!(x)),
            AST::Vlong(ref x)    => write!(f, "#l({})", format_list!(x)),
        }
    }
}

thread_local! {
    pub static _SYMBOLS: UnsafeCell<Vec<String>> = UnsafeCell::new(Vec::new());
    pub static _ENVIRONMENT: UnsafeCell<FnvHashMap<usize, AST>>
                             = UnsafeCell::new(FnvHashMap::with_capacity_and_hasher(10000, Default::default()));
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

pub fn insert_entry(sym: usize, ast: AST) { unsafe { _ENVIRONMENT.with(|e| { (*e.get()).insert(sym, ast) }); } }

pub fn entry(sym: usize) -> Value {
    unsafe {
        _ENVIRONMENT.with(|e| {
            (*e.get()).get(&sym).map(|a| a.clone()).ok_or_else(|| eval_error!("undefined symbol:", symbol_to_str(sym)))
        })
    }
}

fn init_builtin_symbol(sym: &str, ast: AST) {
    let id = new_symbol(sym.to_string());
    insert_entry(id, ast);
}

pub fn init_builtin_symbols() {
    init_builtin_symbol("NIL",  NIL!());
    init_builtin_symbol("T",    T!());
    init_builtin_symbol("plus", FUNCTION!(function::plus));
    init_builtin_symbol("setq", SPECIAL!(special::setq));
}

