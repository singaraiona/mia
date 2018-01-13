#![feature(iterator_try_fold)]

#[macro_use]
extern crate nom;

pub mod ast;
pub mod parser;
pub mod interpreter;
pub mod function;
pub mod special;
