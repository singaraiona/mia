extern crate mia;
#[macro_use]
extern crate nom;

use nom::IResult;
use std::io::{self, Read, Write};
use mia::parser;
use mia::interpreter;
use std::cell::UnsafeCell;

fn ps1() { print!(": "); io::stdout().flush().unwrap(); }

fn main() {
    let mut input   = UnsafeCell::new(vec![0u8;4096]);
    let mut parsed  = vec![];
    let mut size;
    ps1();
    loop {
        size = unsafe { io::stdin().read(&mut (*input.get())).expect("STDIN error.") };
        loop {
            match unsafe { parser::parse(&(*input.get())[..size]) } {
                IResult::Done(i, a) => {
                    //if a.is_empty() && parsed.is_empty() && i.len() > 1 {
                         //unsafe { print!("Bad input: `{}`", ::std::str::from_utf8_unchecked(i)); }
                         //break;
                    //}
                    if a.is_empty() && parsed.is_empty() && i.len() == 1 {
                         break;
                    }
                    parsed.extend(a);
                    if i.is_empty() {
                        match interpreter::fold_list(parsed.as_slice()) {
                            Ok(e)  => println!("-> {}", e),
                            Err(e) => println!("-> {}", e)
                        }
                        parsed.clear();
                        break;
                    }
                    for (j, v) in i.iter().enumerate() { unsafe { (*input.get())[j] = *v; } }
                    let remain = i.len();
                    let inp = unsafe { &mut (*input.get())[remain..] };
                    size = io::stdin().read(inp).expect("STDIN error.") + remain;
                },
                _ => {
                    println!("error");
                    break;
                },
            }
        }
        ps1();
    }
}
