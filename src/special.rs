use mia::*;

pub fn quote(args: &[AST]) -> Result<AST, Error> {
    match args.len() {
        1 => Ok(args[0].clone()),
        _ => Ok(AST::List(Box::new(args.to_vec())))
    }
}
