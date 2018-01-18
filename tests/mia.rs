#![feature(test)]
extern crate test;
#[macro_use]
extern crate mia;
extern crate nom;
//
use nom::IResult;
use std::io::{self, Read, Write};
use mia::parser;
use mia::eval;
use mia::context::Context;
use mia::mia::AST;
//
macro_rules! test {
    ($name:tt, $lhs:expr, $rhs:expr) => {
        #[test]
        pub fn $name() {
            let mut ctx = Context::new();
            match parser::parse($lhs.as_bytes()) {
                IResult::Done(_, a) => assert_eq!(format!("{}",
                                            eval::fold_list(a.as_slice(), &mut ctx).unwrap()), $rhs),
                IResult::Error(e)   => panic!(format!("{:?}", e)),
                _                   => panic!("unknown error."),
            }
        }
    }
}
//-------
test!(atom, "1", "1");
test!(list, "'(1 2)", "(1 2)");
test!(vector, "#l(1 2 3)", "#l(1 2 3)");
test!(string, "\"FOObar123\"","\"FOObar123\"");
test!(plus,  "(+ 1 2 3)", "6");
//test!(minus, "(- 1 2)", "-1");
test!(setq,  "(setq t (+ 1 2)) t", "3");
test!(eq, "(= 1 1)", "T");
test!(ifcond, "(if (= 1 1) T NIL)", "T");
