#![allow(dead_code)]

use super::{
	super::ast::{keyword::Keyword, literal::Literal, punctuation::Punctuation},
	token::{GenericToken, KeywordToken, LiteralToken, PunctuationToken, Span},
};
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

pub fn detect_punctuation(input: Span) -> IResult<Span, PunctuationToken> {
	let (tail, punc) = alt((
		value(
			Punctuation::BracetOpen,
			tag(Punctuation::BracetOpen.as_str()),
		),
		value(
			Punctuation::BracetClose,
			tag(Punctuation::BracetOpen.as_str()),
		),
		value(
			Punctuation::BracetCurlyOpen,
			tag(Punctuation::BracetOpen.as_str()),
		),
		value(
			Punctuation::BracetCurlyClose,
			tag(Punctuation::BracetOpen.as_str()),
		),
		value(Punctuation::Equal, tag(Punctuation::Equal.as_str())),
		value(
			Punctuation::GreaterThan,
			tag(Punctuation::GreaterThan.as_str()),
		),
		value(Punctuation::LessThan, tag(Punctuation::LessThan.as_str())),
		value(
			Punctuation::ExclamationMark,
			tag(Punctuation::ExclamationMark.as_str()),
		),
	))(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		PunctuationToken {
			position: pos,
			token: punc,
		},
	))
}

pub fn decimal(input: Span) -> IResult<Span, LiteralToken> {
	let (tail, token) = recognize(many1(terminated(digit1, many0(char('_')))))(input)?;
	let (tail, pos) = position(tail)?;

	match token.fragment().replace('_', "").parse::<f64>() {
		Ok(number) => Ok((
			tail,
			LiteralToken {
				position: pos,
				token: Literal::Number(number),
			},
		)),
		Err(_err) => Err(nom::Err::Error(nom::error::Error::new(
			tail,
			nom::error::ErrorKind::Float,
		))),
	}
}

pub fn boolean(input: Span) -> IResult<Span, LiteralToken> {
	let (tail, bool_value) = alt((value(true, tag("true")), value(false, tag("false"))))(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		LiteralToken {
			position: pos,
			token: Literal::Bool(bool_value),
		},
	))
}

pub fn undefined(input: Span) -> IResult<Span, LiteralToken> {
	let (tail, _token) = tag("undefined")(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		LiteralToken {
			position: pos,
			token: Literal::Undefined,
		},
	))
}

pub fn null(input: Span) -> IResult<Span, LiteralToken> {
	let (tail, _token) = tag("null")(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		LiteralToken {
			position: pos,
			token: Literal::Null,
		},
	))
}

pub fn string(input: Span) -> IResult<Span, LiteralToken> {
	let (tail, token) = alt((
		delimited(char('\''), take_until("'"), char('\'')),
		delimited(char('"'), take_until("\""), char('"')),
	))(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		LiteralToken {
			position: pos,
			token: Literal::String(token.fragment()),
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
