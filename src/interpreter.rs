use ast::*;

pub fn fold_list(list: &[AST]) -> Result<AST, Error> { list.iter().try_fold(AST::Nil, |acc, x| eval(x.clone())) }
pub fn eval_list(list: &[AST]) -> Result<AST, Error> {
    let l: Result<Vec<AST>, Error> = list.iter().cloned().map(|x| eval(x)).collect();
    Ok(AST::List(Box::new(l?)))
}

pub fn eval(ast: AST) -> Result<AST, Error> {
    if let &AST::List(ref l) = &ast {
        if l.is_empty() { return Ok(AST::Nil); }
        if let AST::Function(f) = eval(l[0].clone())? {
            let args = eval_list(&l[1..])?;
            return (f)(args);
        }
    }
    Ok(ast)
}
