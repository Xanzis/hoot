// scheme values as produced by the hoot parser
// values bound for the interpreter should have
// object references rather than owned allocations
// TODO: write SchemeValue and object storage
// TODO: consider converting to CStrings

#[derive(Clone, Debug)]
pub enum SchemeParserValue {
    Atom(String),
    IntNumber(isize),
    Boolean(bool),
    String(String),
    Character(char),
    DottedPair(Box<SchemeParserValue>, Box<SchemeParserValue>),
    List(Vec<SchemeParserValue>),
}

impl SchemeParserValue {
    pub fn atom<T: ToString>(x: T) -> Self {
        Self::Atom(x.to_string())
    }

    pub fn int_number(x: isize) -> Self {
        Self::IntNumber(x)
    }

    pub fn boolean(x: bool) -> Self {
        Self::Boolean(x)
    }

    pub fn character(x: char) -> Self {
        Self::Character(x)
    }

    pub fn string<T: IntoIterator<Item = char>>(x: T) -> Self {
        Self::String(x.into_iter().collect())
    }

    pub fn list(x: Vec<SchemeParserValue>) -> Self {
        Self::List(x)
    }

    pub fn pair(x: SchemeParserValue, y: SchemeParserValue) -> Self {
        Self::DottedPair(Box::new(x), Box::new(y))
    }

    pub fn left_nested(x: Vec<SchemeParserValue>, y: SchemeParserValue) -> Self {
        // scheme spec includes this form
        Self::DottedPair(Box::new(Self::list(x)), Box::new(y))
    }
}
