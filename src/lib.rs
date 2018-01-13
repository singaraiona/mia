#![feature(iterator_try_fold)]

#[macro_use]
extern crate nom;

#[macro_use]
pub mod mia;
pub mod parser;
pub mod interpreter;
pub mod function;
pub mod special;
