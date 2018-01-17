use mia::*;
use eval::*;
use std::time::Instant;

pub fn quote(args: &[AST]) -> Value {
    match args.len() {
        1 => Ok(args[0].clone()),
        _ => Ok(LIST!(args.to_vec()))
    }
}

pub fn setq(args: &[AST]) -> Value {
    match (&args[0], &args[1]) {
        (&AST::Symbol(l), rhs) => { insert_entry(l as usize, eval(rhs.clone())?); Ok(rhs.clone()) },
        _                      => { args_err!(args)                                               },
    }
}

pub fn de(args: &[AST]) -> Value {
    insert_entry(args[0].symbol(), LAMBDA!(args[1].list().to_vec(), args[2..].to_vec()));
    Ok(args[0].clone())
}

pub fn time(args: &[AST]) -> Value {
    let ts = Instant::now();
    let _ = eval(args[0].clone());
    let now = ts.elapsed();
    let secs = now.as_secs() as i64 * 1000_000_000;
    let nsecs = now.subsec_nanos() as i64;
    Ok(long!(((secs + nsecs) / 1000_000) as i64))
}

pub fn ifcond(args: &[AST]) -> Value {
    if !eval(args[0].clone())?.is_nil() {
        eval(args[1].clone())
    } else { eval(args[2].clone()) }
}

pub fn each(args: &[AST]) -> Value {
    push_frame();
    let prms  = args[0].list();
    let syms  = args[1].list();



    pop_frame();
    nyi_err!()
}
