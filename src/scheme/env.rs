use std::collections::HashMap;
use std::fmt;

use super::builtin;

// wrapper types for environment-managed allocation and storage

#[derive(Clone, Copy, Debug)]
pub struct ObjectReference(usize);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct StringReference(usize);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct SymbolReference(usize);

// general object type, all non-copy underlying data is in the Environment

#[derive(Clone, Copy, Debug)]
pub enum Object {
	Nil,
	Cons(ObjectReference, ObjectReference), // second reference should be to Nil or Cons
	Func(ObjectReference, ObjectReference), // first reference is to argument symbol list, second is to list of expressions
	Val(Value),
}

// consider allocating strings in env and (non-rust) references in Value
#[derive(Clone, Copy, Debug)]
pub enum Value {
    Keyword(builtin::Keyword),
    StandardFunc(builtin::StandardFunc),
    Symbol(SymbolReference),
    IntNumber(isize),
    Boolean(bool),
    String(StringReference),
    Character(char),
}

#[derive(Clone, Debug)]
pub struct Environment {
	symbol_names: HashMap<String, SymbolReference>,
    objects: Vec<Object>,
	strings: HashMap<StringReference, String>,
    vars: Vec<HashMap<SymbolReference, ObjectReference>>,
}

impl Environment {
	pub fn new() -> Self {
		Self {
			symbol_names: HashMap::new(),
			objects: Vec::new(),
			strings: HashMap::new(),
			vars: Vec::new(),
		}
	}

	pub fn push_symbol(&mut self, name: String) -> Value {
		// push a symbol name, returning either a symbol identifier or a keyword (in case of builtin)
		if let Some(k) = builtin::Keyword::get(&name) {
			Value::Keyword(k)
		} else if let Some(f) = builtin::StandardFunc::get(&name) {
			Value::StandardFunc(f)
		} else {
			let next_num = self.symbol_names.len();
			let sref = self.symbol_names.entry(name).or_insert(SymbolReference(next_num));
			Value::Symbol(*sref)
		}
	}

	pub fn find_symbol(&self, sref: SymbolReference) -> String {
		// find a symbol's name. only for use in debug contexts, performance isn't an issue
		self.symbol_names.iter().find(|(_, v)| **v == sref).map(|(k, _)| k.clone()).unwrap()
	}

	pub fn push_object(&mut self, obj: Object) -> ObjectReference {
		let i = self.objects.len();
		self.objects.push(obj);
		ObjectReference(i)
	}

	pub fn get_object(&self, oref: ObjectReference) -> Object {
		// orefs issued by the env are valid between GC passes
		// objects are Copy
		self.objects[oref.0]
	}

	pub fn push_string(&mut self, st: String) -> StringReference {
		let i = self.strings.len();
		let sref = StringReference(i);
		self.strings.insert(sref, st);
		sref
	}

	pub fn get_string(&self, sref: StringReference) -> String {
		self.strings.get(&sref).unwrap().clone()
	}

	pub fn enter_scope(&mut self) {
		// add a new layer of local variables
		self.vars.push(HashMap::new());
	}

	pub fn exit_scope(&mut self) {
		self.vars.pop();
	}

	pub fn set_var(&mut self, sym: SymbolReference, val: Object) {
		let oref = self.push_object(val);
		self.set_var_with_ref(sym, oref);
	}

	pub fn set_var_with_ref(&mut self, sym: SymbolReference, val: ObjectReference) {
		for scope in self.vars.iter_mut().rev() {
			if let Some(v) = scope.get_mut(&sym) {
				*v = val;
			}
		}

		self.vars.last_mut().unwrap().insert(sym, val);
	}

	pub fn get_var(&mut self, sym: SymbolReference) -> Result<ObjectReference, String> {
		for scope in self.vars.iter_mut().rev() {
			if let Some(v) = scope.get(&sym) {
				return Ok(*v)
			}
		}

		// TODO: proper error types / variants
		Err(format!("undefined identifier: {}", self.find_symbol(sym)))
	}

	pub fn push_list(&mut self, vals: Vec<ObjectReference>) -> ObjectReference {
		// allocate and fill a cons list, returning a reference to the head
		// populate the environment with a list
		// TODO shouldn't need to allocate a new Nil for every list
		let nil_ref = self.push_object(Object::Nil);
		let mut cur_ref = nil_ref;

		for oref in vals.into_iter().rev() {
			// push a cons of the new object and the list tail (cur_ref)
			let new_ref = self.push_object(Object::Cons(oref, cur_ref));
			cur_ref = new_ref;
		}

		cur_ref
	}
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	write!(f, "Environment:\n\tSymbol Names:\n")?;
    	for (i, (s, sref)) in self.symbol_names.iter().enumerate() {
    		write!(f, "\t\t{:04} {}: {:?}\n", i, s, sref)?;
    	}

    	write!(f, "\tObjects:\n")?;
    	for (i, o) in self.objects.iter().enumerate() {
    		write!(f, "\t\t{:04} {:?}\n", i, o)?;
    	}

    	write!(f, "\tStrings:\n")?;
    	for (i, (sref, s)) in self.strings.iter().enumerate() {
    		write!(f, "\t\t{:04} {:?}: {}\n", i, sref, s)?;
    	}

    	write!(f, "\tVariables (by scope):\n")?;
    	for (i, scope) in self.vars.iter().enumerate() {
    		for _ in 0..i {
    			write!(f, " ")?;
    		}
    		write!(f, "\t\tScope level {}", i)?;

    		for (sref, oref) in scope.iter() {
    			for _ in 0..i {
	    			write!(f, " ")?;
	    		}

	    		write!(f, "\t\t{:?}: {:?}", sref, oref)?;
    		}
    	}

        Ok(())
    }
}