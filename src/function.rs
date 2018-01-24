use dynasmrt::{self, DynasmApi, DynasmLabelApi};
use mia::*;
use parser;
use std::io::Write;
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use nom::IResult;
use eval;
use context::Context;
use jit;

//macro_rules! fun1 {
    //($name:tt, $lambda:expr) => {
        //pub fn $name(_args: &[AST]) -> Value {
            //if _args.len() != 2 { return arity_err!(2, _args.len()); }
            //$lambda(&_args[0])
        //}
    //}
//}

pub fn plus(args: &[AST], ctx: &mut Context) -> Value {
    match (&args[0], &args[1]) {
        (&AST::Long(l), &AST::Long(r)) => {
            if let Some(ref mut buf) = ctx.jitbl {
            println!("JIT!");
                let plus_fn: jit::JitDyad<i64> = unsafe { mem::transmute(buf.as_ptr()) };
                Ok(long!(plus_fn(l, r)))
            } else {
                let mut jit = dynasmrt::x64::Assembler::new();
                jit::plus_i64(&mut jit);
                let buf = jit.finalize().unwrap();
                let plus_fn: jit::JitDyad<i64> = unsafe { mem::transmute(buf.as_ptr()) };
                ctx.jitbl = Some(buf);
                Ok(long!(plus_fn(l, r)))
            }
        }
        _ => args_err!(args),
    }
    //args.iter().cloned().fold(Ok(NIL!()), |acc, x|
        //match (acc, x) {
                //(Ok(NIL!()),       AST::Long(v)) => Ok(long!(v)),
                //(Ok(AST::Long(u)), AST::Long(v)) => Ok(long!(u + v)),
                //_                                => args_err!(args),
        //})
}

pub fn minus(args: &[AST], ctx: &mut Context) -> Value {
    nyi_err!()
}

pub fn times(args: &[AST], ctx: &mut Context) -> Value {
    nyi_err!()
}

pub fn divide(args: &[AST], ctx: &mut Context) -> Value {
    nyi_err!()
}

pub fn equal(args: &[AST], ctx: &mut Context) -> Value { if args[0] == args[1] { Ok(T!()) } else { Ok(NIL!()) } }

//fun1!(til, |a: &AST| {
    //let vec = (0..a.long()).map(|x| x as i64).collect::<Vec<i64>>();
    //Ok(LONG!(vec))
//});

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

fn pretty(args: &[AST]) -> String {
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
