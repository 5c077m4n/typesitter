use super::{
	scanner::scan,
	token::{
		keyword::Keyword,
		literal::Literal,
		punctuation::Punctuation,
		token_variance::TokenType,
	},
};
use anyhow::Result;
use macros::test_with_logger;

#[test_with_logger]
fn scan_basic_number_init_test() -> Result<()> {
	let input = b"const a = 123;";
	let token_values: Vec<TokenType> = scan(input, Some("test".to_owned()))
		.map(|t| t.value)
		.collect();

	assert_eq!(
		token_values,
		vec![
			TokenType::Keyword(Keyword::Const),
			TokenType::Identifier(b"a"),
			TokenType::Punctuation(Punctuation::Equal),
			TokenType::Literal(Literal::Number(123.)),
			TokenType::Punctuation(Punctuation::Semicolon),
		]
	);
	Ok(())
}

#[test_with_logger]
fn scan_typed_number_init_test() -> Result<()> {
	let input = b"const a: number = 123;";
	let token_values: Vec<TokenType> = scan(input, Some("test".to_owned()))
		.map(|t| t.value)
		.collect();

	assert_eq!(
		token_values,
		vec![
			TokenType::Keyword(Keyword::Const),
			TokenType::Identifier(b"a"),
			TokenType::Punctuation(Punctuation::Colon),
			TokenType::Identifier(b"number"),
			TokenType::Punctuation(Punctuation::Equal),
			TokenType::Literal(Literal::Number(123.)),
			TokenType::Punctuation(Punctuation::Semicolon),
		]
	);
	Ok(())
}

#[test_with_logger]
fn scan_basic_string_init_test() -> Result<()> {
	let input = b"const a = 'what?!';";
	let token_values: Vec<TokenType> = scan(input, Some("test".to_owned()))
		.map(|t| t.value)
		.collect();

	assert_eq!(
		token_values,
		vec![
			TokenType::Keyword(Keyword::Const),
			TokenType::Identifier(b"a"),
			TokenType::Punctuation(Punctuation::Equal),
			TokenType::Literal(Literal::String(b"what?!")),
			TokenType::Punctuation(Punctuation::Semicolon),
		]
	);
	Ok(())
}

#[test_with_logger]
fn scan_console_log_test() -> Result<()> {
	let input = b"console.log(123);";
	let token_values: Vec<TokenType> = scan(input, Some("test".to_owned()))
		.map(|t| t.value)
		.collect();

	assert_eq!(
		token_values,
		vec![
			TokenType::Identifier(b"console"),
			TokenType::Punctuation(Punctuation::Dot),
			TokenType::Identifier(b"log"),
			TokenType::Punctuation(Punctuation::BracketOpen),
			TokenType::Literal(Literal::Number(123.)),
			TokenType::Punctuation(Punctuation::BracketClose),
			TokenType::Punctuation(Punctuation::Semicolon),
		]
	);
	Ok(())
}
