extern crate mia;
extern crate nom;

use nom::IResult;
use std::io::{self, Read, Write};
use mia::parser;
use mia::polyad;
use mia::mia::AST;
use mia::context::Context;

fn ps1() { print!(": "); io::stdout().flush().unwrap(); }

fn main() {
    debug_assert!(::std::mem::size_of::<AST>() == 16, "Sizeof AST is not 16.");
    let mut input = vec![0u8;4096];
    let mut ctx = Context::new();
    ps1();
    loop {
        let size = io::stdin().read(&mut input).expect("STDIN error.");
        match parser::parse(&input[..size]) {
            IResult::Done(_, a) => {
                match polyad::fold_list(a.as_ptr(), a.len(), &mut ctx) {
                    Ok(e)  => println!("-> {}", e),
                    Err(e) => println!("-> {}", e)
                }
            }
            IResult::Error(e) => println!("-> {:?}", e),
            _ => println!("-> Error"),
        }
        ps1();
    }
}
