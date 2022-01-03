use super::env::{Environment, Object, ObjectReference, Value};

// naive interpretation, without tail recursion and many other features
// TODO improve structure, add error handling
pub fn eval(env: &mut Environment, oref: ObjectReference) -> ObjectReference {
    // evaluate the given expression (obj), returning a reference to the result

    // both Object and ObjectReference have Copy semantics
    let obj = env.get_object(oref);

    if !obj.is_list() {
        // return the identity
        return oref;
    }

    let (func, args) = obj.as_list().unwrap();
    let func = env.get_object(func);

    if let Object::Val(Value::Keyword(k)) = func {
        k.apply(env, args)
    } else if let Object::Val(Value::StandardFunc(f)) = func {
        let evaluated = eval_args(env, args);
        f.apply(env, evaluated)
    } else if let Object::Val(Value::Symbol(s)) = func {
        let oref = env.get_var(s).unwrap();

        // retrieve the function
        let obj = env.get_object(oref);
        if let Object::Func(syms, expr) = obj {
            let evaluated = eval_args(env, args);
            apply_func(syms, expr, evaluated)
        } else {
            panic!(
                "symbol {:?}, with value {:?}, is not a function",
                env.find_symbol(s),
                obj
            );
        }
    } else {
        panic!("{:?} is not a function", func);
    }
}

pub fn apply_func(
    arg_syms: ObjectReference,
    expr: ObjectReference,
    args: ObjectReference,
) -> ObjectReference {
    // arg_syms is a list of binding symbols, expr is the function body, args is a list of passed args
    unimplemented!()
}

pub fn eval_args(env: &mut Environment, args: ObjectReference) -> ObjectReference {
    // evaluate a list of arguments, allocating a new list to store their results
    // TODO reduce intermediate allocations maybe, take out panics

    let mut cur = Some(args);
    let mut evaluated = Vec::new();

    while let Some(c) = cur {
        match env.get_object(c) {
            Object::Cons(orefa, orefb) => {
                evaluated.push(eval(env, orefa));
                cur = Some(orefb);
            }
            Object::Nil => cur = None,
            x => panic!("non-list passed to eval_args: {:?}", x),
        }
    }

    env.push_list(evaluated)
}
