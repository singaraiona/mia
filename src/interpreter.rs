use ast::*;

pub fn plus(args: &[AST])-> Result<AST, Error> {
    Err(Error::Nyi)
}

pub fn eval(args: &AST) -> Result<AST, Error> {
    match *ast {
        AST::List(ref l) => {
            match l.len() {
                0 => Ok(AST::Nil),
                _ => {
                    let mut r = eval(&l[0])?;
                    match r {
                        AST::Long(x) => {
                            let mut v = vec![r];
                            for i in 1..l.len() {
                                v.push(eval(&l[i])?);
                            }
                            Ok(AST::List(Box::new(v)))
                        }
                        AST::Verb(v) => {
                            match v {
                                Verb::Plus => plus(&l[1..]),
                                Verb::Quote => Ok(AST::List(Box::new(l[1..].to_vec()))),
                                _ => unimplemented!(),
                            }
                        }
                        _ => {
                            for i in 1..l.len() {
                                r = eval(&l[i])?;
                            }
                            Ok(r)
                        }
                    }
                }
            }
        }
        ref a => Ok(a.clone()),
    }
}
