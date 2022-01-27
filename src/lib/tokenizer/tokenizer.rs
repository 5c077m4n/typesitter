#![allow(dead_code)]

use super::{
	super::ast::{keyword::Keyword, literal::Literal, punctuation::Punctuation},
	token::{Span, Token, TokenType},
};
use nom::{
	branch::alt,
	bytes::complete::{tag, take_until},
	character::complete::{alphanumeric1, char, digit1, multispace1, one_of, space1},
	combinator::{map_res, recognize, value},
	multi::{many0, many1},
	sequence::{delimited, preceded, terminated},
	IResult,
};
use nom_locate::position;

pub fn keyword(input: Span) -> IResult<Span, Token> {
	let (tail, kw) = alt((
		value(
			Keyword::Const,
			terminated(tag(Keyword::Const.as_str()), space1),
		),
		value(Keyword::Let, terminated(tag(Keyword::Let.as_str()), space1)),
		value(Keyword::If, tag(Keyword::If.as_str())),
		value(Keyword::Else, tag(Keyword::Else.as_str())),
		value(Keyword::Function, tag(Keyword::Function.as_str())),
		value(Keyword::Class, tag(Keyword::Class.as_str())),
		value(Keyword::Import, tag(Keyword::Import.as_str())),
		value(Keyword::Export, tag(Keyword::Export.as_str())),
		value(Keyword::Return, tag(Keyword::Return.as_str())),
	))(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		Token {
			position: pos,
			value: TokenType::Keyword(kw),
		},
	))
}

pub fn punctuation(input: Span) -> IResult<Span, Token> {
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
		value(Punctuation::Colon, tag(Punctuation::Colon.as_str())),
		value(Punctuation::Semicolon, tag(Punctuation::Semicolon.as_str())),
	))(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		Token {
			position: pos,
			value: TokenType::Punctuation(punc),
		},
	))
}

pub fn decimal(input: Span) -> IResult<Span, Token> {
	let (tail, token) = map_res(
		recognize(many1(terminated(digit1, many0(char('_'))))),
		|token: Span| token.fragment().replace('_', "").parse::<f64>(),
	)(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		Token {
			position: pos,
			value: TokenType::Literal(Literal::Number(token)),
		},
	))
}

pub fn boolean(input: Span) -> IResult<Span, Token> {
	let (tail, bool_value) = alt((value(true, tag("true")), value(false, tag("false"))))(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		Token {
			position: pos,
			value: TokenType::Literal(Literal::Bool(bool_value)),
		},
	))
}

pub fn undefined(input: Span) -> IResult<Span, Token> {
	let (tail, _token) = tag("undefined")(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		Token {
			position: pos,
			value: TokenType::Literal(Literal::Undefined),
		},
	))
}

pub fn null(input: Span) -> IResult<Span, Token> {
	let (tail, _token) = tag("null")(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		Token {
			position: pos,
			value: TokenType::Literal(Literal::Null),
		},
	))
}

pub fn string(input: Span) -> IResult<Span, Token> {
	let (tail, token) = alt((
		delimited(char('\''), take_until("'"), char('\'')),
		delimited(char('"'), take_until("\""), char('"')),
	))(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		Token {
			position: pos,
			value: TokenType::Literal(Literal::String(token.fragment())),
		},
	))
}

pub fn binary(input: Span) -> IResult<Span, Token> {
	let (tail, token) = preceded(
		tag("0b"),
		recognize(many1(terminated(one_of("01"), many0(char('_'))))),
	)(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		Token {
			position: pos,
			value: TokenType::Generic(&token),
		},
	))
}

pub fn identifier(input: Span) -> IResult<Span, Token> {
	let (tail, token) = alphanumeric1(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		Token {
			position: pos,
			value: TokenType::Identifier(&token),
		},
	))
}

pub fn space(input: Span) -> IResult<Span, Token> {
	let (tail, token) = multispace1(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		Token {
			position: pos,
			value: TokenType::Generic(&token),
		},
	))
}

pub fn all_tokens(input: Span) -> IResult<Span, Token> {
	alt((
		keyword,
		punctuation,
		undefined,
		null,
		decimal,
		string,
		binary,
		space,
		identifier,
	))(input)
}
