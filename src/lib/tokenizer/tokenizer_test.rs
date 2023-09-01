use super::{super::ast::keyword::Keyword, token::*, tokenizer::*};
use anyhow::Result;

#[test]
fn detect_const_test() -> Result<()> {
	let input = Span::new_extra("const", None);
	let (_, KeywordToken { token, position }) = detect_init_keyword(input)?;

	assert_eq!(token, Keyword::Const);
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn detect_let_test() -> Result<()> {
	let input = Span::new_extra("let", None);
	let (_, KeywordToken { token, position }) = detect_init_keyword(input)?;

	assert_eq!(token, Keyword::Let);
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn decimal_test() -> Result<()> {
	let origin = "42";
	let input = Span::new_extra(origin, None);
	let (_, GenericToken { token, position }) = decimal(input)?;

	assert_eq!(token, origin);
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn binary_with_dashes_test() -> Result<()> {
	let origin = "0b01_01";
	let input = Span::new_extra(origin, None);
	let (_, GenericToken { token, position }) = binary(input)?;

	assert_eq!(token, "01_01");
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn binary_without_dashes_test() -> Result<()> {
	let origin = "0b0101";
	let input = Span::new_extra(origin, None);
	let (_, GenericToken { token, position }) = binary(input)?;

	assert_eq!(token, "0101");
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn string_single_quote_test() -> Result<()> {
	let origin = r#"'234'"#;
	let input = Span::new_extra(origin, None);
	let (_, GenericToken { token, position }) = string(input)?;

	assert_eq!(token, "234");
	assert_eq!(position.location_line(), 1);
	Ok(())
}

#[test]
fn string_double_quote_test() -> Result<()> {
	let origin = r#""2 34""#;
	let input = Span::new_extra(origin, None);
	let (_, GenericToken { token, position }) = string(input)?;

	assert_eq!(token, "2 34");
	assert_eq!(position.location_line(), 1);
	Ok(())
}
