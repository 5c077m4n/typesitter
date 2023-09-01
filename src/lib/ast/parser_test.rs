use anyhow::Result;

use super::super::{
	ast::{
		parser::parse,
		types::{
			literal::Literal,
			node::Node,
			var_dec::{VarDec, VarType},
		},
	},
	lexer::scanner::scan,
};

#[test]
pub fn parse_const_number_init_number_test() -> Result<()> {
	let init_str = "const n: number = 123;";
	let tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = *parse(tokens)?;

	assert_eq!(
		ast,
		Node::Block(Box::new(vec![Node::VarDec(Box::new(VarDec {
			var_type: VarType::Const,
			name: "n",
			type_annotation: Some("number"),
			value: Box::new(Node::Literal(Box::new(Literal::Number(123.))))
		}))]))
	);
	Ok(())
}

#[test]
pub fn parse_let_number_init_string_test() -> Result<()> {
	let init_str = r#"let s: string = '123';"#;
	let tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = *parse(tokens)?;

	assert_eq!(
		ast,
		Node::Block(Box::new(vec![Node::VarDec(Box::new(VarDec {
			var_type: VarType::Let,
			name: "s",
			type_annotation: Some("string"),
			value: Box::new(Node::Literal(Box::new(Literal::String("123"))))
		}))]))
	);
	Ok(())
}
