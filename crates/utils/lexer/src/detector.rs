use anyhow::Result;
use nom::{
	branch::alt,
	bytes::complete::{tag, take_until},
	character::complete::{
		alpha1,
		alphanumeric1,
		char,
		digit1,
		line_ending,
		multispace1,
		one_of,
		space1,
		tab,
	},
	combinator::{map_res, recognize, value},
	multi::{many0, many1, many_m_n},
	sequence::{delimited, preceded, terminated, tuple},
	IResult,
};
use nom_locate::position;

use super::token::{
	keyword::Keyword,
	literal::Literal,
	punctuation::Punctuation,
	token_variance::{Span, Token, TokenType},
};

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
		value(
			Keyword::Class,
			terminated(tag(Keyword::Class.as_str()), space1),
		),
		value(
			Keyword::Import,
			terminated(tag(Keyword::Import.as_str()), space1),
		),
		value(
			Keyword::Export,
			terminated(tag(Keyword::Export.as_str()), space1),
		),
		value(
			Keyword::Return,
			terminated(tag(Keyword::Return.as_str()), space1),
		),
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
			Punctuation::BracketOpen,
			tag(Punctuation::BracketOpen.as_str()),
		),
		value(
			Punctuation::BracketClose,
			tag(Punctuation::BracketClose.as_str()),
		),
		value(
			Punctuation::BracketSquareOpen,
			tag(Punctuation::BracketSquareOpen.as_str()),
		),
		value(
			Punctuation::BracketSquareClose,
			tag(Punctuation::BracketSquareClose.as_str()),
		),
		value(
			Punctuation::BracketCurlyOpen,
			tag(Punctuation::BracketCurlyOpen.as_str()),
		),
		value(
			Punctuation::BracketCurlyClose,
			tag(Punctuation::BracketCurlyClose.as_str()),
		),
		value(Punctuation::Equal, tag(Punctuation::Equal.as_str())),
		value(
			Punctuation::GreaterThan,
			tag(Punctuation::GreaterThan.as_str()),
		),
		value(Punctuation::LessThan, tag(Punctuation::LessThan.as_str())),
		value(
			Punctuation::QuoteDouble,
			tag(Punctuation::QuoteDouble.as_str()),
		),
		value(
			Punctuation::QuoteSingle,
			tag(Punctuation::QuoteSingle.as_str()),
		),
		value(
			Punctuation::ExclamationMark,
			tag(Punctuation::ExclamationMark.as_str()),
		),
		value(Punctuation::Colon, tag(Punctuation::Colon.as_str())),
		value(Punctuation::Semicolon, tag(Punctuation::Semicolon.as_str())),
		value(Punctuation::Dot, tag(Punctuation::Dot.as_str())),
		value(Punctuation::Comma, tag(Punctuation::Comma.as_str())),
		value(Punctuation::Space, tag(Punctuation::Space.as_str())),
		value(Punctuation::Tab, tab),
		value(Punctuation::EOL, line_ending),
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
		recognize(tuple((
			many_m_n(0, 1, char('-')),
			many1(terminated(digit1, many0(char('_')))),
			many_m_n(
				0,
				1,
				tuple((char('.'), many1(terminated(digit1, many0(char('_')))))),
			),
		))),
		|token: Span| -> Result<f64> {
			let n_str = token
				.fragment()
				.iter()
				.copied()
				.filter(|c| *c != b'_')
				.collect::<Vec<_>>();
			let n_str = std::str::from_utf8(&n_str[..])?;
			let n = n_str.parse::<f64>()?;
			Ok(n)
		},
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
	let (tail, undefined_literal) = value(Literal::Undefined, tag("undefined"))(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		Token {
			position: pos,
			value: TokenType::Literal(undefined_literal),
		},
	))
}

pub fn null(input: Span) -> IResult<Span, Token> {
	let (tail, null_literal) = value(Literal::Null, tag("null"))(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		Token {
			position: pos,
			value: TokenType::Literal(null_literal),
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
			value: TokenType::Literal(Literal::String(&token)),
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
	let (tail, token) = recognize(tuple((
		many1(alt((alpha1, tag("_"), tag("$")))),
		many0(alt((alphanumeric1, tag("_"), tag("$")))),
	)))(input)?;
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

pub fn empty(input: Span) -> IResult<Span, Token> {
	let (tail, _token) = tag("")(input)?;
	let (tail, pos) = position(tail)?;

	Ok((
		tail,
		Token {
			position: pos,
			value: TokenType::Empty,
		},
	))
}

pub fn all_tokens(input: Span) -> IResult<Span, Token> {
	// The order here is **critical** as the `alt` function runs them in the order that they appear in
	alt((
		decimal,
		string,
		binary,
		keyword,
		punctuation,
		undefined,
		null,
		space,
		identifier,
		empty,
	))(input)
}
