use parser;
use context::Context;
use mia::*;
use std::io::Write;
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use nom::IResult;
use eval::eval;

pub extern "win64" fn fold_list(offset: *const AST, len: usize, ctx: &mut Context) -> Value {
    let list= unsafe { ::std::slice::from_raw_parts(offset, len) };
    list.iter().try_fold(NIL!(), |_, x| eval(x, ctx))
}

pub extern "win64" fn til(offset: *const AST, len: usize, ctx: &mut Context) -> Value {
    let args = unsafe { ::std::slice::from_raw_parts(offset, len) };
    let to = args[0].long();
    let vec = (0..to).map(|x| x as i64).collect::<Vec<i64>>();
    Ok(LONG!(vec))
}

pub extern "win64" fn prin(offset: *const AST, len: usize, ctx: &mut Context) -> Value {
    let args = unsafe { ::std::slice::from_raw_parts(offset, len) };
    print!("{}", args.iter().map(|a| a.to_string()).collect::<String>());
    Ok(T!())
}

pub extern "win64" fn prinl(offset: *const AST, len: usize, ctx: &mut Context) -> Value {
    let args = unsafe { ::std::slice::from_raw_parts(offset, len) };
    println!("{}", args.iter().map(|a| a.to_string()).collect::<String>());
    Ok(T!())
}

fn pretty(args: &[AST]) -> String {
    args.iter().map(|a| {
         match a {
             &AST::List(ref l) if l.len() > FMT_ITEMS_LIMIT => { format!("({}\n    {})", l[0], pretty(&l[1..])) }
             x => format!("{}", x),
         }
    }).collect::<Vec<String>>().join("\n")
}

pub extern "win64" fn pp(offset: *const AST, len: usize, ctx: &mut Context) -> Value {
    let args = unsafe { ::std::slice::from_raw_parts(offset, len) };
    let s = pretty(args);
    println!("{}", s);
    Ok(args[0].clone())
}

pub extern "win64" fn load(offset: *const AST, len: usize, ctx: &mut Context) -> Value {
    let args = unsafe { ::std::slice::from_raw_parts(offset, len) };
    let fl = args[0].string();
    let mut file = File::open(fl).map_err(|_| io_error!(fl, "does not exist."))?;
    let mut input = String::new();
    let size = file.read_to_string(&mut input).map_err(|_| io_error!(fl, "is not readable."))?;
    match parser::parse(input[..size].as_bytes()) {
        IResult::Done(_, a) => fold_list(a.as_ptr(), a.len(), ctx),
        IResult::Error(e) => io_err!(format!("{:?}", e)),
        _ => io_err!("unknown error."),
    }
}
