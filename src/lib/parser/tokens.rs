#![allow(dead_code)]

use nom::{
	branch::alt,
	bytes::complete::{tag, take_until},
	character::complete::{alphanumeric1, char, digit1, multispace0, multispace1, one_of},
	combinator::{opt, recognize},
	multi::{many0, many1, separated_list0},
	sequence::{delimited, preceded, terminated, tuple},
	IResult,
};

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
		delimited(char('\''), take_until("'"), char('\'')),
		delimited(char('"'), take_until("\""), char('"')),
	))(input)
}

pub fn var_init(input: &str) -> IResult<&str, (&str, &str, Option<&str>, &str)> {
	let (tail, (var_init_type, _, var_name, var_type, _, expr, _)) = tuple((
		alt((tag("const"), tag("let"))),
		multispace1,
		alphanumeric1,
		opt(tuple((
			delimited(multispace0, char(':'), multispace0),
			take_until("="),
		))),
		delimited(multispace0, char('='), multispace0),
		take_until(";"),
		char(';'),
	))(input)?;
	let var_type = var_type.map(|vt| vt.1.trim());

	Ok((tail, (var_init_type, var_name, var_type, expr)))
}

pub fn fn_init(input: &str) -> IResult<&str, (Option<&str>, Vec<&str>, Option<&str>, &str)> {
	let (tail, (_, _, fn_name, params, type_out, _, body)) = tuple((
		tag("function"),
		multispace0,
		opt(alphanumeric1),
		delimited(
			char('('),
			separated_list0(char(','), alphanumeric1),
			char(')'),
		),
		opt(tuple((char(':'), multispace0, alphanumeric1, multispace0))),
		multispace0,
		delimited(char('{'), take_until("}"), char('}')),
	))(input)?;
	let type_out = type_out.map(|t| t.2);

	Ok((tail, (fn_name, params, type_out, body)))
}
