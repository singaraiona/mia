use mia::*;

pub fn plus(args: AST) -> Result<AST, Error> {
    match args {
        AST::List(l) => l.iter()
            .cloned()
            .fold(Ok(NIL!()), |acc, x| match (acc, x) {
                (Ok(NIL!()), AST::Long(v)) => Ok(long!(v)),
                (Ok(AST::Long(u)), AST::Long(v)) => Ok(long!(u + v)),
                _ => eval_err!("plus: invalid args."),
            }),
        _ => eval_err!("plus: nyi."),
    }
}

pub fn minus(args: AST) -> Result<AST, Error> {
    eval_err!("nyi")
}

pub fn times(args: AST) -> Result<AST, Error> {
    eval_err!("nyi")
}

pub fn divide(args: AST) -> Result<AST, Error> {
    eval_err!("nyi")
}

