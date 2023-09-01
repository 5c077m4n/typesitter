#![allow(dead_code)]

use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{char, digit1, multispace0, one_of},
	combinator::recognize,
	error::ParseError,
	multi::{many0, many1},
	sequence::{delimited, preceded, terminated},
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
		alt((tag("0b"), tag("0B"))),
		recognize(many1(terminated(one_of("01"), many0(char('_'))))),
	)(input)
}

pub fn decimal(input: &str) -> IResult<&str, &str> {
	recognize(many1(terminated(digit1, many0(char('_')))))(input)
}
