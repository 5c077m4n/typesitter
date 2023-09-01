use lexer::scanner::scan;

use super::{
	super::types::{
		fn_call::FnCall,
		fn_dec::{FnDecl, FnType},
		literal::Literal,
		node::Node,
		type_annotation::TypeAnnotation,
		var_decl::{VarDecl, VarType},
	},
	parse::Parser,
};

macro_rules! parser_test {
	($name:ident, $input:literal, $num_errors:literal) => {
		#[test]
		fn $name() {
			use env_logger;
			let _ = env_logger::builder().is_test(true).try_init();

			let tokens = scan($input, Some("Parser test".to_string()));

			let mut parser = Parser::new(tokens);
			let _ast = parser.parse().unwrap();

			assert_eq!(
				parser.get_errors().len(),
				$num_errors,
				"The wrong number of errors was detected ({:#?})",
				parser.get_errors()
			);
		}
	};
	($name:ident, $input:literal, $test_nodes:expr) => {
		#[test]
		fn $name() {
			use env_logger;
			let _ = env_logger::builder().is_test(true).try_init();

			let tokens = scan($input, Some("Parser test".to_string()));

			let mut parser = Parser::new(tokens);
			let ast = parser.parse().unwrap();

			assert_eq!(*ast, $test_nodes);
			assert_eq!(
				parser.get_errors().len(),
				0,
				"There should be no errors in parsing ({:#?})",
				parser.get_errors()
			);
		}
	};
}

parser_test!(
	parse_const_init_number,
	b"const n: number = 123;",
	vec![Node::VarDecl(VarDecl {
		var_type: VarType::Const,
		name: vec!["n"],
		type_annotation: Some(TypeAnnotation::Number),
		value: Box::new(Node::Literal(Literal::Number(123.)))
	})]
);

parser_test!(
	parse_2_const_init_number,
	b"const n: number = 123;
	const n: number = 123;",
	vec![
		Node::VarDecl(VarDecl {
			var_type: VarType::Const,
			name: vec!["n"],
			type_annotation: Some(TypeAnnotation::Number),
			value: Box::new(Node::Literal(Literal::Number(123.)))
		}),
		Node::VarDecl(VarDecl {
			var_type: VarType::Const,
			name: vec!["n"],
			type_annotation: Some(TypeAnnotation::Number),
			value: Box::new(Node::Literal(Literal::Number(123.)))
		}),
	]
);

parser_test!(
	parse_let_init_string,
	b"let s: string = '123';",
	vec![Node::VarDecl(VarDecl {
		var_type: VarType::Let,
		name: vec!["s"],
		type_annotation: Some(TypeAnnotation::String),
		value: Box::new(Node::Literal(Literal::String(b"123"))),
	})]
);

parser_test!(
	parse_let_number_init_no_type,
	b"let x = 42",
	vec![Node::VarDecl(VarDecl {
		var_type: VarType::Let,
		name: vec!["x"],
		type_annotation: None,
		value: Box::new(Node::Literal(Literal::Number(42.)))
	})]
);

parser_test!(
	parse_let_no_int_value,
	b"let x;",
	vec![Node::VarDecl(VarDecl {
		var_type: VarType::Let,
		name: vec!["x"],
		type_annotation: None,
		value: Box::new(Node::Literal(Literal::Undefined))
	})]
);

parser_test!(
	parse_empty_fn_call,
	b"fnName();",
	vec![Node::FnCall(FnCall {
		fn_name: vec![b"fnName"],
		params: Vec::new(),
	})]
);

parser_test!(
	parse_2_param_fn_call,
	b"fnName(param1, param2);",
	vec![Node::FnCall(FnCall {
		fn_name: vec![b"fnName"],
		params: vec![
			VarDecl {
				var_type: VarType::Let,
				name: vec!["param1"],
				type_annotation: None,
				value: Box::new(Node::Literal(Literal::Undefined))
			},
			VarDecl {
				var_type: VarType::Let,
				name: vec!["param2"],
				type_annotation: None,
				value: Box::new(Node::Literal(Literal::Undefined)),
			}
		],
	})]
);

parser_test!(
	parse_fn_declaration_empty_body_empty_params,
	b"function fn1() {}",
	vec![Node::FnDecl(FnDecl {
		fn_type: FnType::Classic,
		name: Some(vec![b"fn1"]),
		input_params: vec![],
		return_type: None,
		body: Box::new(Node::Block(Vec::new())),
	})]
);

parser_test!(
	parse_fn_decl_empty_body,
	b"function fn1(param1) {}",
	vec![Node::FnDecl(FnDecl {
		fn_type: FnType::Classic,
		name: Some(vec![b"fn1"]),
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

parser_test!(
	parse_fn_declaration_empty_params,
	b"function fn1() {
            const a: string = '123';
        }",
	vec![Node::FnDecl(FnDecl {
		fn_type: FnType::Classic,
		name: Some(vec![b"fn1"]),
		input_params: Vec::new(),
		return_type: None,
		body: Box::new(Node::Block(vec![Node::VarDecl(VarDecl {
			var_type: VarType::Const,
			name: vec!["a"],
			type_annotation: Some(TypeAnnotation::String),
			value: Box::new(Node::Literal(Literal::String(b"123"))),
		})])),
	})]
);

parser_test!(
	parse_anonymous_fn_declaration,
	b"function () {}",
	vec![Node::FnDecl(FnDecl {
		fn_type: FnType::Classic,
		name: None,
		input_params: vec![],
		return_type: None,
		body: Box::new(Node::Block(vec![])),
	})]
);

parser_test!(
	parse_fn_declaration_return_type,
	b"function fn(): void {}",
	vec![Node::FnDecl(FnDecl {
		fn_type: FnType::Classic,
		name: Some(vec![b"fn"]),
		input_params: vec![],
		return_type: Some(TypeAnnotation::Void),
		body: Box::new(Node::Block(vec![])),
	})]
);

parser_test!(
	parse_empty_console_log,
	b"console.log();",
	vec![Node::FnCall({
		FnCall {
			fn_name: vec![b"console", b"log"],
			params: vec![],
		}
	})]
);

parser_test!(
	parse_fn_declaration_and_call,
	b"function fn1(param1: any): void {
        const a: string = '123';
        const b: number = 123;
    }
    fn1();",
	vec![
		Node::FnDecl(FnDecl {
			fn_type: FnType::Classic,
			name: Some(vec![b"fn1"]),
			input_params: vec![VarDecl {
				var_type: VarType::Let,
				name: vec!["param1"],
				type_annotation: Some(TypeAnnotation::Any),
				value: Box::new(Node::Literal(Literal::Undefined))
			}],
			return_type: Some(TypeAnnotation::Void),
			body: Box::new(Node::Block(vec![
				Node::VarDecl(VarDecl {
					var_type: VarType::Const,
					name: vec!["a"],
					type_annotation: Some(TypeAnnotation::String),
					value: Box::new(Node::Literal(Literal::String(b"123"))),
				}),
				Node::VarDecl(VarDecl {
					var_type: VarType::Const,
					name: vec!["b"],
					type_annotation: Some(TypeAnnotation::Number),
					value: Box::new(Node::Literal(Literal::Number(123.))),
				})
			])),
		}),
		Node::FnCall(FnCall {
			fn_name: vec![b"fn1"],
			params: Vec::new()
		})
	]
);

parser_test!(
	parse_fn_declaration_full,
	b"function fn1(param1: any): void {
        const a: string = '123';
        const b: number = 123;
    }",
	vec![Node::FnDecl(FnDecl {
		fn_type: FnType::Classic,
		name: Some(vec![b"fn1"]),
		input_params: vec![VarDecl {
			var_type: VarType::Let,
			name: vec!["param1"],
			type_annotation: Some(TypeAnnotation::Any),
			value: Box::new(Node::Literal(Literal::Undefined))
		}],
		return_type: Some(TypeAnnotation::Void),
		body: Box::new(Node::Block(vec![
			Node::VarDecl(VarDecl {
				var_type: VarType::Const,
				name: vec!["a"],
				type_annotation: Some(TypeAnnotation::String),
				value: Box::new(Node::Literal(Literal::String(b"123"))),
			}),
			Node::VarDecl(VarDecl {
				var_type: VarType::Const,
				name: vec!["b"],
				type_annotation: Some(TypeAnnotation::Number),
				value: Box::new(Node::Literal(Literal::Number(123.))),
			})
		])),
	})]
);

parser_test!(
	parse_fn_return_number,
	b"function() {
        return 1;
    }",
	vec![Node::FnDecl(FnDecl {
		fn_type: FnType::Classic,
		name: None,
		input_params: vec![],
		return_type: None,
		body: Box::new(Node::Block(vec![Node::Return(Box::new(Node::Literal(
			Literal::Number(1.)
		)))])),
	})]
);

parser_test!(
	parse_fn_return_string,
	b"function() {
        return 'string';
    }",
	vec![Node::FnDecl(FnDecl {
		fn_type: FnType::Classic,
		name: None,
		input_params: vec![],
		return_type: None,
		body: Box::new(Node::Block(vec![Node::Return(Box::new(Node::Literal(
			Literal::String(b"string")
		)))])),
	})]
);

parser_test!(
	parse_fn_return_param,
	b"function() {
        const a = 1;
        return a;
    }",
	vec![Node::FnDecl(FnDecl {
		fn_type: FnType::Classic,
		name: None,
		input_params: vec![],
		return_type: None,
		body: Box::new(Node::Block(vec![
			Node::VarDecl(VarDecl {
				var_type: VarType::Const,
				name: vec!["a"],
				type_annotation: None,
				value: Box::new(Node::Literal(Literal::Number(1.)))
			}),
			Node::Return(Box::new(Node::VarCall("a")))
		])),
	})]
);

parser_test!(bad_parse_2_param_fn_call, b"fnName(, param1, param2);", 1);
