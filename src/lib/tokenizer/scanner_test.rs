use super::{
	scanner::scan,
	token::{keyword::Keyword, literal::Literal, punctuation::Punctuation, TokenType},
};

#[test]
fn scan_basic_test() -> anyhow::Result<()> {
	let input = r#"const a = 123;"#;
	let token_values = scan(input, Some("test".to_owned()))
		.filter_map(|t| match t.value {
			TokenType::Generic(frag) if frag.trim() == "" => None,
			other => Some(other),
		})
		.collect::<Vec<TokenType>>();

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
fn scan_console_log_test() -> anyhow::Result<()> {
	let input = r#"console.log(123);"#;
	let token_values = scan(input, Some("test".to_owned()))
		.filter_map(|t| match t.value {
			TokenType::Generic(frag) if frag.trim() == "" => None,
			other => Some(other),
		})
		.collect::<Vec<TokenType>>();

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
