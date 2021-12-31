
// scheme values as produced by the hoot parser
// values bound for the interpreter should have
// object references rather than owned allocations
// TODO: write SchemeValue and object storage

#[derive(Clone, Debug)]
pub enum SchemeParserValue {
	Atom(String),
	IntNumber(isize),
}

impl SchemeParserValue {
	pub fn atom<T: ToString>(x: T) -> Self {
		Self::Atom(x.to_string())
	}

	pub fn int_number(x: isize) -> Self {
		Self::IntNumber(x)
	}
}