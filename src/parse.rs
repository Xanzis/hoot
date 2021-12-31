use std::error::Error;

use crate::scheme::SchemeParserValue;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar, digit1, none_of, one_of},
    combinator::map,
    error::ErrorKind,
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

pub fn parse(i: &str) -> Result<SchemeParserValue, Box<dyn Error + '_>> {
    let (rem, dat) = datum(i)?;

    if !rem.is_empty() {
        Err(format!(
            "unexpected character after datum: {}",
            rem.chars().next().unwrap()
        )
        .into())
    } else {
        Ok(dat)
    }
}

fn datum(i: &str) -> IResult<&str, SchemeParserValue> {
    alt((int_number, boolean, character, string, identifier, list))(i)
}

fn identifier(i: &str) -> IResult<&str, SchemeParserValue> {
    // TODO possibly construct this as a single chain of higher-order functions
    // (is there a performance penalty to reconstructing these functions on each call?)

    if let Ok((rem, dat)) = alt::<_, _, (&str, ErrorKind), _>((tag("+"), tag("-"), tag("...")))(i) {
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

fn boolean(i: &str) -> IResult<&str, SchemeParserValue> {
    let (rem, val) = alt((tag("#t"), tag("#f")))(i)?;

    match val {
        "#t" => Ok((rem, SchemeParserValue::boolean(true))),
        "#f" => Ok((rem, SchemeParserValue::boolean(false))),
        _ => unreachable!(),
    }
}

fn character(i: &str) -> IResult<&str, SchemeParserValue> {
    if let Ok((rem, _)) = tag::<_, _, (&str, ErrorKind)>("#\\newline")(i) {
        return Ok((rem, SchemeParserValue::character('\n')));
    }

    if let Ok((rem, _)) = tag::<_, _, (&str, ErrorKind)>("#\\space")(i) {
        return Ok((rem, SchemeParserValue::character('\n')));
    }

    map(preceded(tag("#\\"), anychar), |c: char| {
        SchemeParserValue::character(c)
    })(i)
}

fn string(i: &str) -> IResult<&str, SchemeParserValue> {
    let (rem, cs) = delimited(
        complete::char('"'),
        many0(alt((
            map(tag("\\\""), |_| '\"'),
            map(tag("\\\\"), |_| '\\'),
            none_of("\\\""),
        ))),
        complete::char('"'),
    )(i)?;

    Ok((rem, SchemeParserValue::string(cs)))
}

fn list(i: &str) -> IResult<&str, SchemeParserValue> {
    alt((
        map(
            delimited(
                complete::char('('),
                separated_list0(tag(" "), datum),
                complete::char(')'),
            ),
            |l| SchemeParserValue::list(l),
        ),
        map(
            delimited(
                complete::char('('),
                tuple((
                    separated_list1(tag(" "), datum),
                    preceded(tag(" . "), datum),
                )),
                complete::char(')'),
            ),
            |(a, b)| {
                if a.len() == 1 {
                    SchemeParserValue::pair(a[0].clone(), b)
                } else {
                    SchemeParserValue::left_nested(a, b)
                }
            },
        ),
    ))(i)
}
