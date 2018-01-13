extern crate mia;
#[macro_use]
extern crate nom;

use nom::IResult;
use std::io::{self, Read, Write};
use mia::parser;
use mia::interpreter;

pub fn ps1() { print!(": "); io::stdout().flush().unwrap(); }

fn main() {
    let mut input = vec![0u8; 256];
    ps1();
    loop {
        let size = io::stdin().read(&mut input).expect("STDIN error.");
        match parser::parse(&input[..size]) {
            IResult::Done(_, a) => {
               match interpreter::eval(a) {
                    Ok(e) => println!("-> {}", e),
                    Err(e) => println!("-> {}", e)
                }
            },

            _ => println!("incomplete"),
        }
        ps1();
    }
}
