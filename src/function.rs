use mia::*;

pub fn plus(args: &[AST]) -> Value {
    args.iter().cloned().fold(Ok(NIL!()), |acc, x|
        match (acc, x) {
                (Ok(NIL!()),       AST::Long(v)) => Ok(long!(v)),
                (Ok(AST::Long(u)), AST::Long(v)) => Ok(long!(u + v)),
                _                                => eval_err!("plus: invalid args."),
        })
}

pub fn minus(args: &[AST]) -> Value {
    eval_err!("nyi")
}

pub fn times(args: &[AST]) -> Value {
    eval_err!("nyi")
}

pub fn divide(args: &[AST]) -> Value {
    eval_err!("nyi")
}

