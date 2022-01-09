#![allow(dead_code)]

use nom::{
	branch::alt,
	bytes::complete::{is_not, tag, take_until},
	character::complete::{char, multispace0, one_of},
	combinator::{opt, recognize, value},
	error::ParseError,
	multi::{many0, many1},
	sequence::{delimited, pair, preceded, terminated, tuple},
	IResult,
};

const EOL: &str = if cfg!(windows) { "\r\n" } else { "\n" };

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn rm_surround_ws<'a, F: 'a, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
	F: Fn(&'a str) -> IResult<&'a str, O, E>,
	E: ParseError<&'a str>,
{
	delimited(multispace0, inner, multispace0)
}

pub fn one_line_comment<'a, E>(input: &'a str) -> IResult<&'a str, (), E>
where
	E: ParseError<&'a str>,
{
	value(
		(), // Output is thrown away.
		pair(tag("//"), is_not("\n\r")),
	)(input)
}

pub fn multiline_comment<'a, E>(input: &'a str) -> IResult<&'a str, (), E>
where
	E: ParseError<&'a str>,
{
	value(
		(), // Output is thrown away.
		tuple((tag("/*"), take_until("*/"), tag("*/"))),
	)(input)
}

pub fn binary(input: &str) -> IResult<&str, &str> {
	preceded(
		alt((tag("0b"), tag("0B"))),
		recognize(many1(terminated(one_of("01"), many0(char('_'))))),
	)(input)
}

pub fn float(input: &str) -> IResult<&str, &str> {
	alt((
		// Case one: .42
		recognize(tuple((
			char('.'),
			decimal,
			opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
		))),
		// Case two: 42e42 and 42.42e42
		recognize(tuple((
			decimal,
			opt(preceded(char('.'), decimal)),
			one_of("eE"),
			opt(one_of("+-")),
			decimal,
		))),
		// Case three: 42. and 42.42
		recognize(tuple((decimal, char('.'), opt(decimal)))),
	))(input)
}

pub fn decimal(input: &str) -> IResult<&str, &str> {
	recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}
