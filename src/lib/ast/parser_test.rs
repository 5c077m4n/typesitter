use anyhow::Result;

use crate::lib::{ast::parser::parse, lexer::scanner::scan};

#[test]
#[ignore]
pub fn parse_const_number_init_test() -> Result<()> {
	let init_str = "const n: number = 123;";
	let tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = *parse(tokens);

	assert_eq!(ast, Vec::new());
	Ok(())
}
