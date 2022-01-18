#![allow(dead_code)]

use nom::{
	branch::alt,
	bytes::complete::{tag, take_till},
	character::complete::{alphanumeric1, char, digit1, multispace0, one_of},
	combinator::recognize,
	error::ParseError,
	multi::{many0, many1},
	sequence::{delimited, preceded, terminated, tuple},
	IResult,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn rm_surround_ws<'a, F: 'a, O, E: 'a>(
	inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
	F: Fn(&'a str) -> IResult<&'a str, O, E>,
	E: ParseError<&'a str>,
{
	delimited(multispace0, inner, multispace0)
}

pub fn binary(input: &str) -> IResult<&str, &str> {
	preceded(
		tag("0b"),
		recognize(many1(terminated(one_of("01"), many0(char('_'))))),
	)(input)
}

pub fn decimal(input: &str) -> IResult<&str, &str> {
	recognize(many1(terminated(digit1, many0(char('_')))))(input)
}

pub fn string(input: &str) -> IResult<&str, &str> {
	alt((
		delimited(char('\''), take_till(|c| c == '\''), char('\'')),
		delimited(char('"'), take_till(|c| c == '"'), char('"')),
	))(input)
}

pub fn var_init(input: &str) -> IResult<&str, (&str, &str)> {
	let (tail, (_, var_name, _, expr, _)) = tuple((
		alt((tag("const "), tag("let "))),
		alphanumeric1,
		rm_surround_ws(char('=')),
		take_till(|c| c == ';'),
        char(';'),
	))(input)?;

	Ok((tail, (var_name, expr)))
}
