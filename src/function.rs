use mia::*;
use parser;

pub fn plus(args: &[AST]) -> Value {
    args.iter().cloned().fold(Ok(NIL!()), |acc, x|
        match (acc, x) {
                (Ok(NIL!()),       AST::Long(v)) => Ok(long!(v)),
                (Ok(AST::Long(u)), AST::Long(v)) => Ok(long!(u + v)),
                _                                => args_err!(args),
        })
}

pub fn minus(args: &[AST]) -> Value {
    nyi_err!()
}

pub fn times(args: &[AST]) -> Value {
    nyi_err!()
}

pub fn divide(args: &[AST]) -> Value {
    nyi_err!()
}

pub fn equal(args: &[AST]) -> Value { if args[0] == args[1] { Ok(T!()) } else { Ok(NIL!()) } }

pub fn til(args: &[AST]) -> Value {
    let len = args[0].long();
    let vec = (0..len).map(|x| x as i64).collect::<Vec<i64>>();
    Ok(LONG!(vec))
}

