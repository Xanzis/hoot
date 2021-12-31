use std::error::Error;

use crate::scheme::SchemeParserValue;
use nom::{
	IResult,
	bytes::complete::tag,
	character::complete::{one_of, digit1},
	multi::many0,
	sequence::tuple,
	branch::alt,
	error::ErrorKind
};

pub fn parse(i: &str) -> Result<SchemeParserValue, Box<dyn Error + '_>> {
	let (rem, dat) = datum(i)?;

	if !rem.is_empty() {
		Err(format!("unexpected character after datum: {}", rem.chars().next().unwrap()).into())
	} else {
		Ok(dat)
	}
}

fn datum(i: &str) -> IResult<&str, SchemeParserValue> {
	alt((
		identifier,
		int_number,
	))(i)
}

fn identifier(i: &str) -> IResult<&str, SchemeParserValue> {
	if let Ok((rem, dat)) = alt::<_, _, (&str, ErrorKind), _>((
		tag("+"),
		tag("-"),
		tag("..."),
	))(i) {
		Ok((rem, SchemeParserValue::atom(dat)))
	} else {
		let initial = one_of("abcdefghijklmnopqrstuvwxyz!$%&*/:<=>?~_^");
		let subsequent = one_of("abcdefghijklmnopqrstuvwxyz!$%&*/:<=>?~_^0123456789.+-");

		let (rem, (data, datb)) = tuple((initial, many0(subsequent)))(i)?;
		let mut res = data.to_string();

		for c in datb {
			res.push(c);
		}

		Ok((rem, SchemeParserValue::atom(res)))
	}
}

fn int_number(i: &str) -> IResult<&str, SchemeParserValue> {
	// TODO: variable radix parsing, float parsing, negatives
	let (rem, val) = digit1(i)?;
	Ok((rem, SchemeParserValue::int_number(val.parse().unwrap())))
}