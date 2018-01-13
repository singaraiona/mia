use std::fmt;

#[derive(Debug)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "** error `{}`", self.0)
    }
}

pub fn error(s: &str) -> Error { Error(s.to_string()) }

pub type Function    = fn(AST)    -> Result<AST, Error>;
pub type SpecialForm = fn(&[AST]) -> Result<AST, Error>;

#[derive(Clone)]
pub enum AST {
    Bool(bool),
    Long(i64),
    Float(f64),
    String(Box<String>),
    Symbol(Box<String>),
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
            AST::Symbol(ref x)      => write!(f, "{}", x),
            AST::Function(ref x)    => write!(f, "{:?}", x),
            AST::SpecialForm(ref x) => write!(f, "{:?}", *x as i64),
            AST::List(ref x)        => write!(f, "({})", x.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(" ")),
            AST::VecLong(ref x)     => write!(f, "#l({})", x.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(" ")),
            AST::Nil                => write!(f, "Nil"),
        }
    }
}
