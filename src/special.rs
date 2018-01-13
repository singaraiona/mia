use ast::*;

pub fn quote(args: &[AST]) -> Result<AST, Error> { Ok(AST::List(Box::new(args.to_vec()))) }
