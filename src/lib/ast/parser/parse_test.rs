use super::super::super::{
	ast::{
		parser::parse::parse,
		types::{
			fn_call::FnCall,
			literal::Literal,
			node::Node,
			var_dec::{VarDec, VarType},
		},
	},
	lexer::scanner::scan,
};
use anyhow::Result;

#[test]
pub fn parse_const_init_number_test() -> Result<()> {
	let init_str = "const n: number = 123;";
	let tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = *parse(tokens, None)?;

	assert_eq!(
		ast,
		Node::Block(vec![Node::VarDec(Box::new(VarDec {
			var_type: VarType::Const,
			name: "n",
			type_annotation: Some("number"),
			value: Box::new(Node::Literal(Box::new(Literal::Number(123.))))
		}))])
	);
	Ok(())
}

#[test]
pub fn parse_2_const_init_number_test() -> Result<()> {
	let init_str = r#"const n: number = 123;
    const n: number = 123;
    "#;
	let tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = *parse(tokens, None)?;

	assert_eq!(
		ast,
		Node::Block(vec![
			Node::VarDec(Box::new(VarDec {
				var_type: VarType::Const,
				name: "n",
				type_annotation: Some("number"),
				value: Box::new(Node::Literal(Box::new(Literal::Number(123.))))
			})),
			Node::VarDec(Box::new(VarDec {
				var_type: VarType::Const,
				name: "n",
				type_annotation: Some("number"),
				value: Box::new(Node::Literal(Box::new(Literal::Number(123.))))
			}))
		])
	);
	Ok(())
}

#[test]
pub fn parse_let_init_string_test() -> Result<()> {
	let init_str = r#"let s: string = '123';"#;
	let tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = *parse(tokens, None)?;

	assert_eq!(
		ast,
		Node::Block(vec![Node::VarDec(Box::new(VarDec {
			var_type: VarType::Let,
			name: "s",
			type_annotation: Some("string"),
			value: Box::new(Node::Literal(Box::new(Literal::String("123"))))
		}))])
	);
	Ok(())
}

#[test]
pub fn parse_let_number_init_no_type_test() -> Result<()> {
	let init_str = r#"let x = 42;"#;
	let tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = *parse(tokens, None)?;

	assert_eq!(
		ast,
		Node::Block(vec![Node::VarDec(Box::new(VarDec {
			var_type: VarType::Let,
			name: "x",
			type_annotation: None,
			value: Box::new(Node::Literal(Box::new(Literal::Number(42.))))
		}))])
	);
	Ok(())
}

#[test]
#[ignore]
pub fn parse_let_no_int_value_test() -> Result<()> {
	let init_str = r#"let x;"#;
	let tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = *parse(tokens, None)?;

	assert_eq!(
		ast,
		Node::Block(vec![Node::VarDec(Box::new(VarDec {
			var_type: VarType::Let,
			name: "x",
			type_annotation: None,
			value: Box::new(Node::Literal(Box::new(Literal::Undefined)))
		}))])
	);
	Ok(())
}

#[test]
pub fn parse_empty_fn_call_test() -> Result<()> {
	let init_str = r#"fnName();"#;
	let tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = *parse(tokens, None)?;

	assert_eq!(
		ast,
		Node::Block(vec![Node::FnCall(Box::new(FnCall {
			fn_name: "fnName",
			params: Vec::new(),
		}))])
	);
	Ok(())
}

#[test]
pub fn parse_2_param_fn_call_test() -> Result<()> {
	let init_str = r#"fnName(param_1, param_2);"#;
	let tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = *parse(tokens, None)?;

	assert_eq!(
		ast,
		Node::Block(vec![Node::FnCall(Box::new(FnCall {
			fn_name: "fnName",
			params: vec!["param_1", "param_2"],
		}))])
	);
	Ok(())
}

#[test]
#[should_panic]
pub fn bad_parse_2_param_fn_call_test() {
	let init_str = r#"fnName(, param_1, param_2);"#;
	let tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = *parse(tokens, None).unwrap();

	assert_eq!(
		ast,
		Node::Block(vec![Node::FnCall(Box::new(FnCall {
			fn_name: "fnName",
			params: vec!["param_1", "param_2"],
		}))])
	);
}
