use crate::parse::SchemeParserValue;
use super::env::{Environment, ObjectReference, Object, Value};

// tooling for building an interpreter environment from a parse structure

pub fn build_environment(input: SchemeParserValue) -> (Environment, ObjectReference) {
	// consume a parsed program structure, returning a populated environment
	// and a reference to the head of the program list

	let mut env = Environment::new();
	let program = build_value(&mut env, input);

	(env, program)
}

fn build_value(env: &mut Environment, input: SchemeParserValue) -> ObjectReference {
	// populate the environment with a value, returning a reference to the object
	use SchemeParserValue::*;
	match input {
		Symbol(s) => {
			let sval = env.push_symbol(s);
			env.push_object(Object::Val(sval))
		},
	    IntNumber(i) => {
	    	let ival = Value::IntNumber(i);
	    	env.push_object(Object::Val(ival))
	    },
	    Boolean(b) => {
	    	let bval = Value::Boolean(b);
	    	env.push_object(Object::Val(bval))
	    },
	    String(s) => {
	    	let sref = env.push_string(s);
	    	let sval = Value::String(sref);
	    	env.push_object(Object::Val(sval))
	    },
	    Character(c) => {
	    	let cval = Value::Character(c);
	    	env.push_object(Object::Val(cval))
	    },
	    List(l) => build_list(env, l),
	}
}

fn build_list(env: &mut Environment, input: Vec<SchemeParserValue>) -> ObjectReference {
	// populate the environment with a list
	// TODO shouldn't need to allocate a new Nil for every list
	let orefs = input.into_iter().map(|val| build_value(env, val)).collect();

	env.push_list(orefs)
}