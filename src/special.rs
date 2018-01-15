use mia::*;
use interpreter::*;

pub fn quote(args: &[AST]) -> Value {
    match args.len() {
        1 => Ok(args[0].clone()),
        _ => Ok(LIST!(args.to_vec()))
    }
}

pub fn setq(args: &[AST]) -> Value {
    match (&args[0], &args[1]) {
        (&AST::Symbol(l), rhs) => insert_entry(l as usize, eval((*rhs).clone())?),
        _ => return eval_err!("nyi"),
    }
    Ok(args[1].clone())
}

pub fn de(args: &[AST]) -> Value {
    let lambda = LAMBDA!(args[1].list().to_vec(), args[2..].to_vec());
    insert_entry(args[0].symbol(), lambda);
    Ok(args[0].clone())
}
