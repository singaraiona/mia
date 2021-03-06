use mia::*;
use context::Context;

pub fn fold_list(list: &[AST], ctx: &mut Context) -> Value { list.iter().try_fold(NIL!(), |_, x| eval(x, ctx)) }

pub fn eval_list(list: &[AST], ctx: &mut Context) -> Value {
    Ok(LIST!(list.iter().map(|x| eval(x, ctx)).collect::<Vvalue>()?))
}

pub fn eval(ast: &AST, ctx: &mut Context) -> Value {
    match *ast {
        AST::List(ref l) if !l.is_empty() => {
            match l[0] {
                AST::Symbol(s) => {
                    let e = ctx.entry(s)?.clone();
                    call(&e, &l[1..], ctx)
                },
                ref f          => call(f, &l[1..], ctx),
            }
        }
        AST::Symbol(s) => Ok(ctx.entry(s)?.clone()),
        ref a => Ok(a.clone()),
    }
}

#[inline]
fn call(car: &AST, cdr: &[AST], ctx: &mut Context) -> Value {
    match *car {
        AST::Function(f) => (f)(cdr.iter().map(|x| eval(x, ctx)).collect::<Vvalue>()?.as_slice(), ctx),
        AST::Special(f)  => (f)(cdr, ctx),
        AST::Lambda(box Lambda { ref args, ref body }) => {
            //ctx.push_frame();
            for (s, v) in args.iter().zip(cdr.iter()) {
                let e = eval(v, ctx)?;
                ctx.insert_entry(s.symbol(), e);
            }
            let r = fold_list(body.as_slice(), ctx);
            //ctx.pop_frame();
            r
        },
        AST::Vlong(ref l) => {
            l.get(cdr[0].long() as usize).map(|&x| long!(x))
            .ok_or_else(|| bound_error!(car, format_list!(cdr)))
        },
        ref l @ AST::List(_) => call(&eval(l, ctx)?, cdr, ctx),
        ref x => call_err!(x)
    }
}
