use super::super::super::{
	ast::{
		parser::parse::parse,
		types::{
			fn_call::FnCall,
			fn_dec::{FnDec, FnType},
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
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens, None)?;

	assert_eq!(
		ast,
		vec![Box::new(Node::VarDecl(VarDec {
			var_type: VarType::Const,
			name: "n",
			type_annotation: Some("number"),
			value: Box::new(Node::Literal(Literal::Number(123.)))
		}))]
	);
	Ok(())
}

#[test]
pub fn parse_2_const_init_number_test() -> Result<()> {
	let init_str = r#"const n: number = 123;
    const n: number = 123;
    "#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens, None)?;

	assert_eq!(
		ast,
		vec![
			Box::new(Node::VarDecl(VarDec {
				var_type: VarType::Const,
				name: "n",
				type_annotation: Some("number"),
				value: Box::new(Node::Literal(Literal::Number(123.)))
			})),
			Box::new(Node::VarDecl(VarDec {
				var_type: VarType::Const,
				name: "n",
				type_annotation: Some("number"),
				value: Box::new(Node::Literal(Literal::Number(123.)))
			})),
		]
	);
	Ok(())
}

#[test]
pub fn parse_let_init_string_test() -> Result<()> {
	let init_str = r#"let s: string = '123';"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens, None)?;

	assert_eq!(
		ast,
		vec![Box::new(Node::VarDecl(VarDec {
			var_type: VarType::Let,
			name: "s",
			type_annotation: Some("string"),
			value: Box::new(Node::Literal(Literal::String("123"))),
		}))]
	);
	Ok(())
}

#[test]
pub fn parse_let_number_init_no_type_test() -> Result<()> {
	let init_str = r#"let x = 42;"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens, None)?;

	assert_eq!(
		ast,
		vec![Box::new(Node::VarDecl(VarDec {
			var_type: VarType::Let,
			name: "x",
			type_annotation: None,
			value: Box::new(Node::Literal(Literal::Number(42.)))
		}))]
	);
	Ok(())
}

#[test]
#[ignore]
pub fn parse_let_no_int_value_test() -> Result<()> {
	let init_str = r#"let x;"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens, None)?;

	assert_eq!(
		ast,
		vec![Box::new(Node::VarDecl(VarDec {
			var_type: VarType::Let,
			name: "x",
			type_annotation: None,
			value: Box::new(Node::Literal(Literal::Undefined))
		}))]
	);
	Ok(())
}

#[test]
pub fn parse_empty_fn_call_test() -> Result<()> {
	let init_str = r#"fnName();"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens, None)?;

	assert_eq!(
		ast,
		vec![Box::new(Node::FnCall(FnCall {
			fn_name: "fnName",
			params: Vec::new(),
		}))]
	);
	Ok(())
}

#[test]
pub fn parse_2_param_fn_call_test() -> Result<()> {
	let init_str = r#"fnName(param_1, param_2);"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens, None)?;

	assert_eq!(
		ast,
		vec![Box::new(Node::FnCall(FnCall {
			fn_name: "fnName",
			params: vec![
				VarDec {
					var_type: VarType::Let,
					name: "param_1",
					type_annotation: None,
					value: Box::new(Node::Literal(Literal::Undefined))
				},
				VarDec {
					name: "param_2",
					var_type: VarType::Let,
					type_annotation: None,
					value: Box::new(Node::Literal(Literal::Undefined)),
				}
			],
		}))]
	);
	Ok(())
}

#[test]
#[should_panic]
pub fn bad_parse_2_param_fn_call_test() {
	let init_str = r#"fnName(, param_1, param_2);"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let _ast = parse(&mut tokens, None).unwrap();
}

#[test]
pub fn parse_fn_declaration_empty_body_test() -> Result<()> {
	let init_str = r#"function fn1(param_1) {}"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens, None)?;

	assert_eq!(
		ast,
		vec![Box::new(Node::FnDec(FnDec {
			fn_type: FnType::Classic,
			name: Some("fn1"),
			input_params: vec![VarDec {
				var_type: VarType::Let,
				name: "param_1",
				type_annotation: None,
				value: Box::new(Node::Literal(Literal::Undefined))
			}],
			return_type: None,
			body: Box::new(Node::Block(Vec::new())),
		}))]
	);
	Ok(())
}

#[test]
pub fn parse_fn_declaration_empty_body_empty_params_test() -> Result<()> {
	let init_str = r#"function fn1() {}"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens, None)?;

	assert_eq!(
		ast,
		vec![Box::new(Node::FnDec(FnDec {
			fn_type: FnType::Classic,
			name: Some("fn1"),
			input_params: Vec::new(),
			return_type: None,
			body: Box::new(Node::Block(Vec::new())),
		}))]
	);
	Ok(())
}
