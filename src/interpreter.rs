use mia::*;

pub fn fold_list(list: &[AST]) -> Result<AST, Error> { list.iter().try_fold(AST::Nil, |_, x| eval(x.clone())) }

pub fn eval_list(list: &[AST]) -> Result<AST, Error> {
    let l: Result<Vec<AST>, Error> = list.iter().cloned().map(|x| eval(x)).collect();
    Ok(AST::List(Box::new(l?)))
}

pub fn eval(ast: AST) -> Result<AST, Error> {
    match ast {
        AST::List(ref l) if !l.is_empty() => {
            match l[0] {
                AST::Symbol(s)      => { match entry(s)? {
                                             AST::Function(f)    => { (f)(eval_list(&l[1..])?)           }
                                             AST::SpecialForm(f) => { (f)(&l[1..])                       }
                                             _                   => { eval_err!("car must be callable.") }
                                         }
                                       }
                AST::Function(f)    => { (f)(eval_list(&l[1..])?)           }
                AST::SpecialForm(f) => { (f)(&l[1..])                       }
                _                   => { eval_err!("car must me callable.") }
            }
        }
        AST::Symbol(s) => { entry(s) }
        a => Ok(a),
    }
}
