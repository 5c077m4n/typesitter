use super::{
	super::ast::{keyword::Keyword, literal::Literal},
	token::*,
	tokenizer::*,
};
use anyhow::Result;

#[test]
fn const_kw_test() -> Result<()> {
	let input = Span::new_extra("const", None);
	let (_, KeywordToken { token, position }) = keyword(input)?;

	assert_eq!(token, Keyword::Const);
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn let_kw_test() -> Result<()> {
	let input = Span::new_extra("let", None);
	let (_, KeywordToken { token, position }) = keyword(input)?;

	assert_eq!(token, Keyword::Let);
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn bool_true_test() -> Result<()> {
	let input = Span::new_extra("true", None);
	let (_, LiteralToken { token, position }) = boolean(input)?;

	assert_eq!(token, Literal::Bool(true));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn bool_false_test() -> Result<()> {
	let input = Span::new_extra("false", None);
	let (_, LiteralToken { token, position }) = boolean(input)?;

	assert_eq!(token, Literal::Bool(false));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn decimal_test() -> Result<()> {
	let origin = "42";
	let input = Span::new_extra(origin, None);
	let (_, LiteralToken { token, position }) = decimal(input)?;

	assert_eq!(token, Literal::Number(42.));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn decimal_underscore_test() -> Result<()> {
	let origin = "42_000";
	let input = Span::new_extra(origin, None);
	let (tail, LiteralToken { token, position }) = decimal(input)?;

	assert_eq!(tail.fragment().to_owned(), "");
	assert_eq!(token, Literal::Number(42_000.));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn binary_with_dashes_test() -> Result<()> {
	let origin = "0b01_01";
	let input = Span::new_extra(origin, None);
	let (tail, GenericToken { token, position }) = binary(input)?;

	assert_eq!(tail.fragment().to_owned(), "");
	assert_eq!(token, "01_01");
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn binary_without_dashes_test() -> Result<()> {
	let origin = "0b0101";
	let input = Span::new_extra(origin, None);
	let (tail, GenericToken { token, position }) = binary(input)?;

	assert_eq!(tail.fragment().to_owned(), "");
	assert_eq!(token, "0101");
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn string_single_quote_test() -> Result<()> {
	let origin = r#"'234'"#;
	let input = Span::new_extra(origin, None);
	let (_, LiteralToken { token, position }) = string(input)?;

	assert_eq!(token, Literal::String("234"));
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn string_double_quote_test() -> Result<()> {
	let origin = r#""2 34""#;
	let input = Span::new_extra(origin, None);
	let (_, LiteralToken { token, position }) = string(input)?;

	assert_eq!(token, Literal::String("2 34"));
	assert_eq!(position.location_line(), 1);
	Ok(())
}
