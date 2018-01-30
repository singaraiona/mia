use dynasmrt::{self, DynasmApi, DynasmLabelApi};
use mia::*;
use eval::*;
use std::time::Instant;
use std::mem;
use context::Context;
use jit;

pub fn quote(args: &[AST], ctx: &mut Context) -> Value {
    match args.len() {
        1 => Ok(args[0].clone()),
        _ => Ok(LIST!(args.to_vec()))
    }
}

pub fn setq(args: &[AST], ctx: &mut Context) -> Value {
    let e = eval(&args[1], ctx)?;
    ctx.insert_entry(args[0].symbol(), e);
    Ok(T!())

    //match (&args[0], &args[1]) {
        //(&AST::Symbol(l), rhs) => {
            //let e = eval(rhs, ctx)?;
            //ctx.insert_entry(l as usize, e);
            //Ok(rhs.clone())
        //},
        //_ => { args_err!(args) },
    //}
}

pub fn de(args: &[AST], ctx: &mut Context) -> Value {
    ctx.insert_entry(args[0].symbol(), LAMBDA!(args[1].list().to_vec(), args[2..].to_vec()));
    Ok(args[0].clone())
}

pub fn time(args: &[AST], ctx: &mut Context) -> Value {
    let ts = Instant::now();
    let _ = eval(&args[0], ctx);
    let now = ts.elapsed();
    let secs = now.as_secs() as i64 * 1000_000_000;
    let nsecs = now.subsec_nanos() as i64;
    Ok(long!(((secs + nsecs) / 1000_000) as i64))
}

pub fn ifcond(args: &[AST], ctx: &mut Context) -> Value {
    if !eval(&args[0], ctx)?.is_nil() {
        eval(&args[1], ctx)
    } else { eval(&args[2], ctx) }
}

pub fn forcond(args: &[AST], ctx: &mut Context) -> Value {
    ctx.push_frame();
    // TODO
    ctx.pop_frame();
    nyi_err!()
}

pub fn whilecond(args: &[AST], ctx: &mut Context) -> Value {
    let mut compiler = jit::Compiler::new(ctx);
    let buf = compiler.compile(&args[1]);

    while !eval(&args[0], ctx)?.is_nil() {
        jit_call!(compiler, buf);
    }

    Ok(NIL!())
}
