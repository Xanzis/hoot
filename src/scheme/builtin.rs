use super::env::{Environment, Object, ObjectReference, Value};

// tools for handling scheme builtins

#[derive(Clone, Copy, Debug)]
pub enum Keyword {
    Quote,
    If,
    Set,
    Define,
    Lambda,
    Begin,
}

impl Keyword {
    pub fn get(name: &str) -> Option<Self> {
        use Keyword::*;

        match name {
            "quote" => Some(Quote),
            "if" => Some(If),
            "set!" => Some(Set),
            "define" => Some(Define),
            "lambda" => Some(Lambda),
            "begin" => Some(Begin),
            _ => None,
        }
    }

    pub fn apply(self, env: &mut Environment, args: ObjectReference) -> ObjectReference {
        // unlike other function evaluations, these arguments are passed BEFORE evaluation
        // (because If will only evaluate one arm)
        // args is the head of the argument list

        match self {
            _ => unimplemented!(),
        }
    }
}

// standard library functions (very incomplete for now)
#[derive(Clone, Copy, Debug)]
pub enum StandardFunc {
    Plus,
    Minus,
}

impl StandardFunc {
    pub fn get(name: &str) -> Option<Self> {
        use StandardFunc::*;

        match name {
            "+" => Some(Plus),
            "-" => Some(Minus),
            _ => None,
        }
    }

    pub fn apply(self, env: &mut Environment, args: ObjectReference) -> ObjectReference {
        // here, the arguments have been fully evaluated (as in ordinary function application)
        // TODO proper error handling throughout
        use StandardFunc::*;

        match self {
            Plus => {
                // TODO better error handling, non-int types, edge cases
                let (a, b) = env.list_as_2(args).expect("improper args to plus");
                if let Object::Val(Value::IntNumber(x)) = env.get_object(a) {
                    if let Object::Val(Value::IntNumber(y)) = env.get_object(b) {
                        let obj = Object::Val(Value::IntNumber(x + y));
                        return env.push_object(obj);
                    } else {
                        panic!("{:?} is not a number", env.get_object(b));
                    }
                } else {
                    panic!("{:?} is not a number", env.get_object(a));
                }
            }
            _ => unimplemented!(),
        }
    }
}
