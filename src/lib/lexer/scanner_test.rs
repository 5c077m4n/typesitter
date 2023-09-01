use super::{
	scanner::scan,
	token::{keyword::Keyword, literal::Literal, punctuation::Punctuation, TokenType},
};

#[test]
fn scan_basic_number_init_test() -> anyhow::Result<()> {
	let input = r#"const a = 123;"#;
	let token_values: Vec<TokenType> = scan(input, Some("test".to_owned()))
		.map(|t| t.value)
		.collect();

	assert_eq!(
		token_values,
		vec![
			TokenType::Keyword(Keyword::Const),
			TokenType::Identifier("a"),
			TokenType::Punctuation(Punctuation::Equal),
			TokenType::Literal(Literal::Number(123.)),
			TokenType::Punctuation(Punctuation::Semicolon)
		]
	);
	Ok(())
}

#[test]
fn scan_typed_number_init_test() -> anyhow::Result<()> {
	let input = r#"const a: number = 123;"#;
	let token_values: Vec<TokenType> = scan(input, Some("test".to_owned()))
		.map(|t| t.value)
		.collect();

	assert_eq!(
		token_values,
		vec![
			TokenType::Keyword(Keyword::Const),
			TokenType::Identifier("a"),
			TokenType::Punctuation(Punctuation::Colon),
			TokenType::Identifier("number"),
			TokenType::Punctuation(Punctuation::Equal),
			TokenType::Literal(Literal::Number(123.)),
			TokenType::Punctuation(Punctuation::Semicolon)
		]
	);
	Ok(())
}

#[test]
fn scan_basic_string_init_test() -> anyhow::Result<()> {
	let input = r#"const a = 'what?!';"#;
	let token_values: Vec<TokenType> = scan(input, Some("test".to_owned()))
		.map(|t| t.value)
		.collect();

	assert_eq!(
		token_values,
		vec![
			TokenType::Keyword(Keyword::Const),
			TokenType::Identifier("a"),
			TokenType::Punctuation(Punctuation::Equal),
			TokenType::Literal(Literal::String("what?!")),
			TokenType::Punctuation(Punctuation::Semicolon)
		]
	);
	Ok(())
}

#[test]
fn scan_console_log_test() -> anyhow::Result<()> {
	let input = r#"console.log(123);"#;
	let token_values: Vec<TokenType> = scan(input, Some("test".to_owned()))
		.map(|t| t.value)
		.collect();

	assert_eq!(
		token_values,
		vec![
			TokenType::Identifier("console"),
			TokenType::Punctuation(Punctuation::Dot),
			TokenType::Identifier("log"),
			TokenType::Punctuation(Punctuation::BracketOpen),
			TokenType::Literal(Literal::Number(123.)),
			TokenType::Punctuation(Punctuation::BracketClose),
			TokenType::Punctuation(Punctuation::Semicolon)
		]
	);
	Ok(())
}
