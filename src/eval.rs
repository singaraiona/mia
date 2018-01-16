use mia::*;

pub fn fold_list(list: &[AST]) -> Value { list.iter().try_fold(NIL!(), |_, x| eval(x.clone())) }

pub fn eval_list(list: &[AST]) -> Value { Ok(LIST!(list.iter().cloned().map(|x| eval(x)).collect::<Vvalue>()?)) }

pub fn eval(ast: AST) -> Value {
    match ast {
        AST::List(ref l) if !l.is_empty() => {
            match l[0] {
                AST::Symbol(s) => call(entry(s)?, &l[1..]),
                ref f          => call(f.clone(), &l[1..]),
            }
        }
        AST::Symbol(s) => entry(s),
        a => Ok(a),
    }
}

#[inline]
fn call(car: AST, cdr: &[AST]) -> Value {
    match car {
        AST::Function(f) => (f)(cdr.iter().cloned().map(|x| eval(x)).collect::<Vvalue>()?.as_slice()),
        AST::Special(f)  => (f)(cdr),
        AST::Lambda(box Lambda { ref args, ref body }) => {
            push_frame();
            for (s, v) in args.iter().zip(cdr.iter()) {
                insert_entry(s.symbol(), eval(v.clone())?);
            }
            let r = fold_list(body.as_slice());
            pop_frame();
            r
        },
        AST::Vlong(ref l) => {
            l.get(cdr[0].long() as usize).map(|&x| long!(x))
            .ok_or_else(|| eval_error!("Index out of bounds:", car, format_list!(cdr)))
        },
        l @ AST::List(_) => call(eval(l)?, cdr),
        x => eval_err!("CAR: expected callable, found:", x)
    }
}
