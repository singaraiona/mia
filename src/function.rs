use ast::*;

pub fn plus(args: AST) -> Result<AST, Error> {
    match args {
        AST::List(l) => l.iter()
            .cloned()
            .fold(Ok(AST::Nil), |acc, x| match (acc, x) {
                (Ok(AST::Nil), AST::Long(v)) => Ok(AST::Long(v)),
                (Ok(AST::Long(u)), AST::Long(v)) => Ok(AST::Long(u + v)),
                _ => Err(error("plus: invalid args.")),
            }),
        _ => Err(error("plus: nyi.")),
    }
}

pub fn minus(args: AST) -> Result<AST, Error> {
    Err(error("nyi"))
}

pub fn times(args: AST) -> Result<AST, Error> {
    Err(error("nyi"))
}

pub fn divide(args: AST) -> Result<AST, Error> {
    Err(error("nyi"))
}
