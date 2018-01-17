use mia::*;
use parser;
use std::io::Write;
use std::fs::File;
use std::io::prelude::*;
use nom::IResult;
use eval;

pub fn plus(args: &[AST]) -> Value {
    args.iter().cloned().fold(Ok(NIL!()), |acc, x|
        match (acc, x) {
                (Ok(NIL!()),       AST::Long(v)) => Ok(long!(v)),
                (Ok(AST::Long(u)), AST::Long(v)) => Ok(long!(u + v)),
                _                                => args_err!(args),
        })
}

pub fn minus(args: &[AST]) -> Value {
    nyi_err!()
}

pub fn times(args: &[AST]) -> Value {
    nyi_err!()
}

pub fn divide(args: &[AST]) -> Value {
    nyi_err!()
}

pub fn equal(args: &[AST]) -> Value { if args[0] == args[1] { Ok(T!()) } else { Ok(NIL!()) } }

pub fn til(args: &[AST]) -> Value {
    let len = args[0].long();
    let vec = (0..len).map(|x| x as i64).collect::<Vec<i64>>();
    Ok(LONG!(vec))
}

pub fn prin(args: &[AST]) -> Value {
    print!("{}", args.iter().map(|a| a.to_string()).collect::<String>());
    Ok(args[0].clone())
}

pub fn prinl(args: &[AST]) -> Value {
    println!("{}", args.iter().map(|a| a.to_string()).collect::<String>());
    Ok(args[0].clone())
}

fn pretty(args: &[AST]) -> String {
    args.iter().map(|a| {
         match a {
             &AST::List(ref l) if l.len() > FMT_ITEMS_LIMIT => { format!("({}\n    {})", l[0], pretty(&l[1..])) }
             x => format!("{}", x),
         }
    }).collect::<Vec<String>>().join("\n")
}

pub fn pp(args: &[AST]) -> Value {
    let s = pretty(args);
    println!("{}", s);
    Ok(args[0].clone())
}

pub fn load(args: &[AST]) -> Value {
    let fl = args[0].string();
    let mut file = File::open(fl).map_err(|_| io_error!(fl, "does not exist."))?;
    let mut input = String::new();
    let size = file.read_to_string(&mut input).map_err(|_| io_error!(fl, "is not readable."))?;
    match parser::parse(input[..size].as_bytes()) {
        IResult::Done(_, a) => eval::fold_list(a.as_slice()),
        IResult::Error(e) => io_err!(format!("{:?}", e)),
        _ => io_err!("unknown error."),
    }
}
