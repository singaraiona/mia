use std::fmt;
use std::cell::UnsafeCell;

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
macro_rules! eval_error { ($($x:expr),+) => { Err($crate::mia::Error(error_fmt!($($x),+))) }}

pub type Function    = fn(AST)    -> Result<AST, Error>;
pub type SpecialForm = fn(&[AST]) -> Result<AST, Error>;

#[derive(Clone)]
pub enum AST {
    Bool(bool),
    Long(i64),
    Float(f64),
    String(Box<String>),
    Symbol(usize),
    Function(Function),
    SpecialForm(SpecialForm),
    VecLong(Box<Vec<i64>>),
    List(Box<Vec<AST>>),
    Nil,
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AST::Bool(x)            => if x { write!(f, "#t") } else { write!(f, "#f") },
            AST::Long(ref x)        => write!(f, "{}", x),
            AST::Float(ref x)       => write!(f, "{}", x),
            AST::String(ref x)      => write!(f, "\"{}\"", x),
            AST::Symbol(x)          => write!(f, "{}", symbol_to_str(x)),
            AST::Function(ref x)    => write!(f, "{:?}", x),
            AST::SpecialForm(ref x) => write!(f, "{:?}", *x as i64),
            AST::List(ref x)        => write!(f, "({})", x.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(" ")),
            AST::VecLong(ref x)     => write!(f, "#l({})", x.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(" ")),
            AST::Nil                => write!(f, "Nil"),
        }
    }
}

thread_local! { pub static _SYMBOLS: UnsafeCell<Vec<String>> = UnsafeCell::new(Vec::new()); }

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
