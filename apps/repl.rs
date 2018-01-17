extern crate mia;
extern crate nom;

use nom::IResult;
use std::io::{self, Read, Write};
use mia::parser;
use mia::eval;
use mia::mia::{AST, init_builtin_symbols};

fn ps1() { print!(": "); io::stdout().flush().unwrap(); }

fn main() {
    debug_assert!(::std::mem::size_of::<AST>() == 16, "Sizeof AST is not 16.");
    let mut input = vec![0u8;4096];
    init_builtin_symbols();
    ps1();
    loop {
        let size = io::stdin().read(&mut input).expect("STDIN error.");
        match parser::parse(&input[..size]) {
            IResult::Done(_, a) => {
                match eval::fold_list(a.as_slice()) {
                    Ok(e)  => println!("-> {}", e),
                    Err(e) => println!("-> {}", e)
                }
            }
            IResult::Error(e) => println!("-> {:?}", e),
            _ => {
                println!("-> Error");
            }
        }
        ps1();
    }
}
