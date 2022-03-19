use super::{
	super::types::{
		fn_call::FnCall,
		fn_dec::{FnDec, FnType},
		literal::Literal,
		node::Node,
		var_dec::{VarDecl, VarType},
	},
	parse::parse,
};
use anyhow::Result;
use lexer::scanner::scan;

#[test]
pub fn parse_const_init_number_test() -> Result<()> {
	let init_str = "const n: number = 123;";
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::VarDecl(VarDecl {
			var_type: VarType::Const,
			name: vec!["n"],
			type_annotation: Some("number"),
			value: Box::new(Node::Literal(Literal::Number(123.)))
		})]
	);
	Ok(())
}

#[test]
pub fn parse_2_const_init_number_test() -> Result<()> {
	let init_str = r#"const n: number = 123;
    const n: number = 123;
    "#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![
			Node::VarDecl(VarDecl {
				var_type: VarType::Const,
				name: vec!["n"],
				type_annotation: Some("number"),
				value: Box::new(Node::Literal(Literal::Number(123.)))
			}),
			Node::VarDecl(VarDecl {
				var_type: VarType::Const,
				name: vec!["n"],
				type_annotation: Some("number"),
				value: Box::new(Node::Literal(Literal::Number(123.)))
			}),
		]
	);
	Ok(())
}

#[test]
pub fn parse_let_init_string_test() -> Result<()> {
	let init_str = r#"let s: string = '123';"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::VarDecl(VarDecl {
			var_type: VarType::Let,
			name: vec!["s"],
			type_annotation: Some("string"),
			value: Box::new(Node::Literal(Literal::String("123"))),
		})]
	);
	Ok(())
}

#[test]
pub fn parse_let_number_init_no_type_test() -> Result<()> {
	let init_str = r#"let x = 42;"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::VarDecl(VarDecl {
			var_type: VarType::Let,
			name: vec!["x"],
			type_annotation: None,
			value: Box::new(Node::Literal(Literal::Number(42.)))
		})]
	);
	Ok(())
}

#[test]
pub fn parse_let_no_int_value_test() -> Result<()> {
	let init_str = r#"let x;"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::VarDecl(VarDecl {
			var_type: VarType::Let,
			name: vec!["x"],
			type_annotation: None,
			value: Box::new(Node::Literal(Literal::Undefined))
		})]
	);
	Ok(())
}

#[test]
pub fn parse_empty_fn_call_test() -> Result<()> {
	let init_str = r#"fnName();"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::FnCall(FnCall {
			fn_name: vec!["fnName"],
			params: Vec::new(),
		})]
	);
	Ok(())
}

#[test]
pub fn parse_2_param_fn_call_test() -> Result<()> {
	let init_str = r#"fnName(param1, param2);"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::FnCall(FnCall {
			fn_name: vec!["fnName"],
			params: vec![
				VarDecl {
					var_type: VarType::Let,
					name: vec!["param1"],
					type_annotation: None,
					value: Box::new(Node::Literal(Literal::Undefined))
				},
				VarDecl {
					name: vec!["param2"],
					var_type: VarType::Let,
					type_annotation: None,
					value: Box::new(Node::Literal(Literal::Undefined)),
				}
			],
		})]
	);
	Ok(())
}

#[test]
#[should_panic]
pub fn bad_parse_2_param_fn_call_test() {
	let init_str = r#"fnName(, param1, param2);"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let _ast = parse(&mut tokens).unwrap();
}

#[test]
pub fn parse_fn_declaration_empty_body_empty_params_test() -> Result<()> {
	let init_str = r#"function fn1() {}"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::FnDecl(FnDec {
			fn_type: FnType::Classic,
			name: Some(vec!["fn1"]),
			input_params: Vec::new(),
			return_type: None,
			body: Box::new(Node::Block(Vec::new())),
		})]
	);
	Ok(())
}

#[test]
pub fn parse_fn_declaration_empty_body_test() -> Result<()> {
	let init_str = r#"function fn1(param1) {}"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::FnDecl(FnDec {
			fn_type: FnType::Classic,
			name: Some(vec!["fn1"]),
			input_params: vec![VarDecl {
				var_type: VarType::Let,
				name: vec!["param1"],
				type_annotation: None,
				value: Box::new(Node::Literal(Literal::Undefined))
			}],
			return_type: None,
			body: Box::new(Node::Block(Vec::new())),
		})]
	);
	Ok(())
}

#[test]
pub fn parse_fn_declaration_empty_params_test() -> Result<()> {
	let init_str = r#"function fn1() {
        const a: string = '123';
    }"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::FnDecl(FnDec {
			fn_type: FnType::Classic,
			name: Some(vec!["fn1"]),
			input_params: Vec::new(),
			return_type: None,
			body: Box::new(Node::Block(vec![Node::VarDecl(VarDecl {
				var_type: VarType::Const,
				name: vec!["a"],
				type_annotation: Some("string"),
				value: Box::new(Node::Literal(Literal::String("123"))),
			})])),
		})]
	);
	Ok(())
}

#[test]
pub fn parse_anonymous_fn_declaration_test() -> Result<()> {
	let init_str = r#"function () {}"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::FnDecl(FnDec {
			fn_type: FnType::Classic,
			name: None,
			input_params: vec![],
			return_type: None,
			body: Box::new(Node::Block(vec![])),
		})]
	);
	Ok(())
}

#[test]
pub fn parse_fn_declaration_return_type_test() -> Result<()> {
	let init_str = r#"function fn(): void {}"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::FnDecl(FnDec {
			fn_type: FnType::Classic,
			name: Some(vec!["fn"]),
			input_params: vec![],
			return_type: Some("void"),
			body: Box::new(Node::Block(vec![])),
		})]
	);
	Ok(())
}

#[test]
pub fn parse_empty_console_log_test() -> Result<()> {
	let init_str = r#"console.log();"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::FnCall({
			FnCall {
				fn_name: vec!["console", "log"],
				params: vec![],
			}
		})]
	);
	Ok(())
}

#[test]
pub fn parse_fn_declaration_full_test() -> Result<()> {
	let init_str = r#"function fn1(param1: any): void {
        const a: string = '123';
        const b: number = 123;
    }"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::FnDecl(FnDec {
			fn_type: FnType::Classic,
			name: Some(vec!["fn1"]),
			input_params: vec![VarDecl {
				var_type: VarType::Let,
				name: vec!["param1"],
				type_annotation: Some("any"),
				value: Box::new(Node::Literal(Literal::Undefined))
			}],
			return_type: Some("void"),
			body: Box::new(Node::Block(vec![
				Node::VarDecl(VarDecl {
					var_type: VarType::Const,
					name: vec!["a"],
					type_annotation: Some("string"),
					value: Box::new(Node::Literal(Literal::String("123"))),
				}),
				Node::VarDecl(VarDecl {
					var_type: VarType::Const,
					name: vec!["b"],
					type_annotation: Some("number"),
					value: Box::new(Node::Literal(Literal::Number(123.))),
				})
			])),
		})]
	);
	Ok(())
}

#[test]
pub fn parse_fn_return_number_test() -> Result<()> {
	let init_str = r#"function() {
        return 1;
    }"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::FnDecl(FnDec {
			fn_type: FnType::Classic,
			name: None,
			input_params: vec![],
			return_type: None,
			body: Box::new(Node::Block(vec![Node::Return(Box::new(Node::Literal(
				Literal::Number(1.)
			)))])),
		})]
	);
	Ok(())
}

#[test]
pub fn parse_fn_return_string_test() -> Result<()> {
	let init_str = r#"function() {
        return 'string';
    }"#;
	let mut tokens = scan(init_str, Some("Parser test".to_string()));
	let ast = parse(&mut tokens)?;

	assert_eq!(
		ast,
		vec![Node::FnDecl(FnDec {
			fn_type: FnType::Classic,
			name: None,
			input_params: vec![],
			return_type: None,
			body: Box::new(Node::Block(vec![Node::Return(Box::new(Node::Literal(
				Literal::String("string")
			)))])),
		})]
	);
	Ok(())
}
