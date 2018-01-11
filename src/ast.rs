use std::fmt;

#[derive(Debug)]
pub enum Error {
    Nyi,
}

#[derive(Debug, Clone)]
pub enum Verb {
    Plus,
    Minus,
    Times,
    Divide,
    Quote
}

#[derive(Debug, Clone)]
pub enum Function {
    Quote,
}

#[derive(Debug, Clone)]
pub enum AST {
    Bool(bool),
    Long(i64),
    Float(f64),
    String(Box<String>),
    Symbol(Box<String>),
    Verb(Verb),
    List(Box<Vec<AST>>),
    Nil,
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AST::Bool(x)       => if x { write!(f, "#t") } else { write!(f, "#f") },
            AST::Long(ref x)   => write!(f, "{}", x),
            AST::Float(ref x)  => write!(f, "{}", x),
            AST::String(ref x) => write!(f, "\"{}\"", x),
            AST::Symbol(ref x) => write!(f, "{}", x),
            AST::Verb(ref x)   => write!(f, "{:?}", x),
            AST::List(ref x)   => write!(f, "({})", x.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(" ")),
            AST::Nil           => write!(f, "Nil"),
        }
    }
}
