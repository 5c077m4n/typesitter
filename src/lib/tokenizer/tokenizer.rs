#![allow(dead_code)]

use super::super::ast::{keyword::Keyword, literal::Literal};
use super::token::{GenericToken, KeywordToken, Span};
use nom::{
	branch::alt,
	bytes::complete::{tag, take_until},
	character::complete::{char, digit1, one_of},
	combinator::{recognize, value},
	multi::{many0, many1},
	sequence::{delimited, preceded, terminated},
	IResult,
};
use nom_locate::position;

pub fn detect_init_keyword(input: Span) -> IResult<Span, KeywordToken> {
	let (tail, kw) = alt((
		value(Keyword::Const, tag(Keyword::Const.as_str())),
		value(Keyword::Let, tag(Keyword::Let.as_str())),
	))(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		KeywordToken {
			position: pos,
			token: kw,
		},
	))
}

pub fn binary(input: Span) -> IResult<Span, GenericToken> {
	let (tail, token) = preceded(
		tag("0b"),
		recognize(many1(terminated(one_of("01"), many0(char('_'))))),
	)(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		GenericToken {
			position: pos,
			token: &token,
		},
	))
}

pub fn decimal(input: Span) -> IResult<Span, GenericToken> {
	let (tail, token) = recognize(many1(terminated(digit1, many0(char('_')))))(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		GenericToken {
			position: pos,
			token: &token,
		},
	))
}

pub fn string(input: Span) -> IResult<Span, GenericToken> {
	let (tail, token) = alt((
		delimited(char('\''), take_until("'"), char('\'')),
		delimited(char('"'), take_until("\""), char('"')),
	))(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		GenericToken {
			position: pos,
			token: &token,
		},
	))
}
