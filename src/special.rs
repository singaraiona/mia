use mia::*;

pub fn quote(args: &[AST]) -> Result<AST, Error> {
    match args.len() {
        1 => Ok(args[0].clone()),
        _ => Ok(LIST!(args.to_vec()))
    }
}

pub fn setq(args: &[AST]) -> Result<AST, Error> {
    match (&args[0], &args[1]) {
        (&AST::Symbol(l), rhs) => insert_entry(l as usize, (*rhs).clone()),
        _ => return eval_err!("nyi"),
    }
    Ok(NIL!())
}
