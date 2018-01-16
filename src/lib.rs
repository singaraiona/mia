#![feature(iterator_try_fold)]
#![feature(box_patterns)]
#![feature(core_intrinsics)]

#[macro_use]
extern crate nom;
extern crate fnv;
#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod mia;
pub mod parser;
pub mod eval;
pub mod stack;
pub mod function;
pub mod special;
