#![feature(iterator_try_fold)]
#![feature(box_patterns)]
#![feature(core_intrinsics)]
#![feature(plugin)]
#![plugin(dynasm)]

#[macro_use]
extern crate nom;
extern crate fnv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate dynasmrt;

#[macro_use]
pub mod mia;
pub mod parser;
pub mod jit;
pub mod eval;
pub mod stack;
pub mod context;
pub mod dyad;
pub mod polyad;
pub mod special;
