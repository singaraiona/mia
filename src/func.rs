use ast::*;

pub fn plus(args: AST) -> Result<AST, Error> {
    match args {
        AST::List(l) => {
            l.iter().cloned().fold(Ok(AST::Nil), |acc, x| {
                match (acc, x) {
                    (Ok(AST::Nil), AST::Long(v)) => Ok(AST::Long(v)),
                    (Ok(AST::Long(u)), AST::Long(v)) => Ok(AST::Long(u + v)),
                    _ => Err(Error::Nyi),
                }
            })
        }
        _ => Err(Error::Nyi),
    }
}

pub fn minus(args: AST) -> Result<AST, Error> { Err(Error::Nyi) }
pub fn times(args: AST) -> Result<AST, Error> { Err(Error::Nyi) }
pub fn divide(args: AST) -> Result<AST, Error> { Err(Error::Nyi) }
pub fn quote(args: AST) -> Result<AST, Error> { Ok(args) }
