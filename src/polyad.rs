use parser;
use context::Context;
use mia::*;
use std::io::Write;
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use nom::IResult;
use eval;

pub fn til(args: &[AST], ctx: &mut Context) -> Value {
    let len = args[0].long();
    let vec = (0..len).map(|x| x as i64).collect::<Vec<i64>>();
    Ok(LONG!(vec))
}

pub fn prin(args: &[AST], ctx: &mut Context) -> Value {
    print!("{}", args.iter().map(|a| a.to_string()).collect::<String>());
    Ok(args[0].clone())
}

pub fn prinl(args: &[AST], ctx: &mut Context) -> Value {
    println!("{}", args.iter().map(|a| a.to_string()).collect::<String>());
    Ok(args[0].clone())
}

pub fn pretty(args: &[AST]) -> String {
    args.iter().map(|a| {
         match a {
             &AST::List(ref l) if l.len() > FMT_ITEMS_LIMIT => { format!("({}\n    {})", l[0], pretty(&l[1..])) }
             x => format!("{}", x),
         }
    }).collect::<Vec<String>>().join("\n")
}

pub fn pp(args: &[AST], ctx: &mut Context) -> Value {
    let s = pretty(args);
    println!("{}", s);
    Ok(args[0].clone())
}

pub fn load(args: &[AST], ctx: &mut Context) -> Value {
    let fl = args[0].string();
    let mut file = File::open(fl).map_err(|_| io_error!(fl, "does not exist."))?;
    let mut input = String::new();
    let size = file.read_to_string(&mut input).map_err(|_| io_error!(fl, "is not readable."))?;
    match parser::parse(input[..size].as_bytes()) {
        IResult::Done(_, a) => eval::fold_list(a.as_slice(), ctx),
        IResult::Error(e) => io_err!(format!("{:?}", e)),
        _ => io_err!("unknown error."),
    }
}
