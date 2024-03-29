use anyhow::Result;
use macros::test_with_logger;

use super::{
	detector::{all_tokens, binary, boolean, decimal, identifier, keyword, punctuation, string},
	token::{
		keyword::Keyword,
		literal::Literal,
		token_variance::{Span, Token, TokenType},
	},
};
use crate::token::punctuation::Punctuation;

#[test_with_logger]
fn const_kw_test() -> Result<()> {
	let input = Span::new_extra(b"const a = 'wert';", None);
	let (_, Token { value, position }) = keyword(input)?;

	assert_eq!(value, TokenType::Keyword(Keyword::Const));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn let_kw_test() -> Result<()> {
	let input = Span::new_extra(b"let b = 9;", None);
	let (_, Token { value, position }) = keyword(input)?;

	assert_eq!(value, TokenType::Keyword(Keyword::Let));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn bool_true_test() -> Result<()> {
	let input = Span::new_extra(b"true", None);
	let (_, Token { value, position }) = boolean(input)?;

	assert_eq!(value, TokenType::Literal(Literal::Bool(true)));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn bool_false_test() -> Result<()> {
	let input = Span::new_extra(b"false", None);
	let (_, Token { value, position }) = boolean(input)?;

	assert_eq!(value, TokenType::Literal(Literal::Bool(false)));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn decimal_longer_test() -> Result<()> {
	let origin = b"12345";
	let input = Span::new_extra(origin, None);
	let (_, Token { value, position }) = decimal(input)?;

	assert_eq!(value, TokenType::Literal(Literal::Number(12345.)));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn decimal_test() -> Result<()> {
	let origin = b"42";
	let input = Span::new_extra(origin, None);
	let (_, Token { value, position }) = decimal(input)?;

	assert_eq!(value, TokenType::Literal(Literal::Number(42.)));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn negative_decimal_test() -> Result<()> {
	let origin = b"-42";
	let input = Span::new_extra(origin, None);
	let (_, Token { value, position }) = decimal(input)?;

	assert_eq!(value, TokenType::Literal(Literal::Number(-42.)));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn decimal_test_with_fraction() -> Result<()> {
	let origin = b"42.95";
	let input = Span::new_extra(origin, None);
	let (_, Token { value, position }) = decimal(input)?;

	assert_eq!(value, TokenType::Literal(Literal::Number(42.95)));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn decimal_underscore_test() -> Result<()> {
	let origin = b"42_000";
	let input = Span::new_extra(origin, None);
	let (tail, Token { value, position }) = decimal(input)?;

	assert_eq!(tail.fragment(), b"");
	assert_eq!(value, TokenType::Literal(Literal::Number(42_000.)));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn binary_with_dashes_test() -> Result<()> {
	let origin = b"0b01_01";
	let input = Span::new_extra(origin, None);
	let (tail, Token { value, position }) = binary(input)?;

	assert_eq!(tail.fragment(), b"");
	assert_eq!(value, TokenType::Generic(b"01_01"));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn string_single_quote_test() -> Result<()> {
	let origin = b"'234'";
	let input = Span::new_extra(origin, None);
	let (_, Token { value, position }) = string(input)?;

	assert_eq!(value, TokenType::Literal(Literal::String(b"234")));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn string_double_quote_test() -> Result<()> {
	let origin = b"\"2 34\"";
	let input = Span::new_extra(origin, None);
	let (_, Token { value, position }) = string(input)?;

	assert_eq!(value, TokenType::Literal(Literal::String(b"2 34")));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn binary_without_dashes_test() -> Result<()> {
	let origin = b"0b0101";
	let input = Span::new_extra(origin, None);
	let (tail, Token { value, position }) = binary(input)?;

	assert_eq!(tail.fragment(), b"");
	assert_eq!(value, TokenType::Generic(b"0101"));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn identifier_base_test() -> Result<()> {
	let origin = b"paramName";
	let input = Span::new_extra(origin, None);
	let (tail, Token { value, position }) = identifier(input)?;

	assert_eq!(tail.fragment(), b"");
	assert_eq!(value, TokenType::Identifier(origin));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn identifier_test() -> Result<()> {
	let origin = b"param_name_1";
	let input = Span::new_extra(origin, None);
	let (tail, Token { value, position }) = identifier(input)?;

	assert_eq!(tail.fragment(), b"");
	assert_eq!(value, TokenType::Identifier(origin));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn identifier_dollar_test() -> Result<()> {
	let origin = b"param_name_$";
	let input = Span::new_extra(origin, None);
	let (tail, Token { value, position }) = identifier(input)?;

	assert_eq!(tail.fragment(), b"");
	assert_eq!(value, TokenType::Identifier(origin));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
#[should_panic]
fn identifier_no_digit_at_start_test() {
	let origin = b"1_param_name";
	let input = Span::new_extra(origin, None);
	let (_, Token { .. }) = identifier(input).unwrap();
}

#[test_with_logger]
fn punctuation_space() -> Result<()> {
	let origin = b" ";
	let input = Span::new_extra(origin, None);
	let (tail, Token { value, position }) = punctuation(input)?;

	assert_eq!(tail.fragment(), b"");
	assert_eq!(value, TokenType::Punctuation(Punctuation::Space));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn punctuation_tab() -> Result<()> {
	let origin = b"	";
	let input = Span::new_extra(origin, None);
	let (tail, Token { value, position }) = punctuation(input)?;

	assert_eq!(tail.fragment(), b"");
	assert_eq!(value, TokenType::Punctuation(Punctuation::Tab));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test_with_logger]
fn all_tokens_number_test() -> Result<()> {
	let input = Span::new_extra(b"123", None);
	let (_, Token { value, .. }) = all_tokens(input)?;

	assert_eq!(value, TokenType::Literal(Literal::Number(123.)));
	Ok(())
}

#[test_with_logger]
fn all_tokens_assignment_test() -> Result<()> {
	let input = Span::new_extra(b"const a = \"123\";", None);
	let (_, Token { value, .. }) = all_tokens(input)?;

	assert_eq!(value, TokenType::Keyword(Keyword::Const));
	Ok(())
}
