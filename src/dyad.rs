use mia::*;
use parser;
use nom::IResult;
use eval;
use context::Context;

pub fn plus(lhs: &AST, rhs: &AST, ctx: &mut Context) -> Value {
    match (lhs, rhs) {
        (&AST::Long(l), &AST::Long(r)) => {
            Ok(long!(l + r))
        }
        _ => args_err!(lhs, rhs),
    }
}

pub fn minus(lhs: &AST, rhs: &AST, ctx: &mut Context) -> Value {
    nyi_err!()
}

pub fn equal(lhs: &AST, rhs: &AST, ctx: &mut Context) -> Value {
    if lhs == rhs { Ok(T!()) } else { Ok(NIL!()) }
}
