use ast::*;

pub fn fold_list(list: &[AST]) -> Result<AST, Error> { list.iter().try_fold(AST::Nil, |_, x| eval(x.clone())) }

pub fn eval_list(list: &[AST]) -> Result<AST, Error> {
    let l: Result<Vec<AST>, Error> = list.iter().cloned().map(|x| eval(x)).collect();
    Ok(AST::List(Box::new(l?)))
}

pub fn eval(ast: AST) -> Result<AST, Error> {
    match &ast {
        &AST::List(ref l) if !l.is_empty() => {
            match l[0] {
                AST::Symbol(ref s)  => { return Err(error("symbol eval: nyi."));     }
                AST::Function(f)    => { return (f)(eval_list(&l[1..])?);            }
                AST::SpecialForm(f) => { return (f)(&l[1..]);                        }
                _                   => { return Err(error("car must me callable.")); }
            }
        }
        &AST::Symbol(ref s) => { return Err(error("symbol eval: nyi.")); }
        _ => {},
    }
    Ok(ast)
}
