extern crate mia;
extern crate nom;

use nom::IResult;
use std::io::{self, Read, Write};
use mia::parser;
use mia::interpreter;
use mia::mia::init_builtin_symbols;
use std::cell::UnsafeCell;

fn ps1() { print!("mia> "); io::stdout().flush().unwrap(); }

fn main() {
    let mut input = vec![0u8;4096];
    init_builtin_symbols();
    ps1();
    loop {
        let size = io::stdin().read(&mut input).expect("STDIN error.");
        match parser::parse(&input[..size]) {
            IResult::Done(_, a) => {
                match interpreter::fold_list(a.as_slice()) {
                    Ok(e)  => println!("{}", e),
                    Err(e) => println!("{}", e)
                }
            }
            IResult::Error(e) => println!("{:?}", e),
            _ => {
                println!("error");
            }
        }
        ps1();
    }
}
