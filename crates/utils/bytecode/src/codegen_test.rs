use anyhow::Result;
use ast::types::{
	fn_call::FnCall,
	literal::Literal,
	node::Node,
	type_annotation::TypeAnnotation,
	var_decl::{VarDecl, VarType},
};

use super::{codegen::CodeGen, instr::Instr};

#[test]
fn empty_program() -> Result<()> {
	let tree = Node::Block(vec![Node::Block(vec![])]);
	let mut codegen = CodeGen::default();
	let prog = codegen.run(&tree)?;

	assert_eq!(prog, vec![]);
	Ok(())
}

#[test]
fn long_number_decl() -> Result<()> {
	let tree = Node::Block(vec![Node::VarDecl(VarDecl {
		var_type: VarType::Const,
		name: vec!["a"],
		type_annotation: Some(TypeAnnotation::Number),
		value: Box::new(Node::Literal(Literal::Number(12345.))),
	})]);
	let mut codegen = CodeGen::default();
	let prog = codegen.run(&tree)?;

	assert_eq!(prog, vec![Instr::Push(12345.)]);
	Ok(())
}

#[test]
fn number_decl() -> Result<()> {
	let tree = Node::Block(vec![Node::VarDecl(VarDecl {
		var_type: VarType::Const,
		name: vec!["param_1"],
		type_annotation: None,
		value: Box::new(Node::Literal(Literal::Number(1.))),
	})]);
	let mut codegen = CodeGen::default();
	let prog = codegen.run(&tree)?;

	assert_eq!(prog, vec![Instr::Push(1.)]);
	Ok(())
}

#[test]
fn number_decl_and_call() -> Result<()> {
	let tree = Node::Block(vec![
		Node::VarDecl(VarDecl {
			var_type: VarType::Const,
			name: vec!["param_1"],
			type_annotation: None,
			value: Box::new(Node::Literal(Literal::Number(1.))),
		}),
		Node::VarCall("param_1"),
	]);
	let mut codegen = CodeGen::default();
	let prog = codegen.run(&tree)?;

	assert_eq!(prog, vec![Instr::Push(1.), Instr::Get(0)]);
	Ok(())
}

#[test]
#[should_panic = "Could not find the requested variable `does_not_exist`"]
fn number_decl_and_call_wrong_param() {
	let tree = Node::Block(vec![
		Node::VarDecl(VarDecl {
			var_type: VarType::Const,
			name: vec!["param_1"],
			type_annotation: None,
			value: Box::new(Node::Literal(Literal::Number(1.))),
		}),
		Node::VarCall("does_not_exist"),
	]);
	let mut codegen = CodeGen::default();
	let _prog = codegen.run(&tree).unwrap();
}

#[test]
fn builtin_console_log() -> Result<()> {
	let tree = Node::Block(vec![
		Node::VarDecl(VarDecl {
			var_type: VarType::Const,
			name: vec!["param_1"],
			type_annotation: None,
			value: Box::new(Node::Literal(Literal::Number(1.))),
		}),
		Node::FnCall(FnCall {
			fn_name: vec![b"console", b"log"],
			params: vec![VarDecl {
				var_type: VarType::Const,
				name: vec!["param_1"],
				type_annotation: None,
				value: Box::new(Node::Literal(Literal::Number(1.))),
			}],
		}),
	]);
	let mut codegen = CodeGen::default();
	let prog = codegen.run(&tree).unwrap();

	assert_eq!(
		prog,
		vec![
			Instr::Push(1.0),
			Instr::CallBuiltin("console.log".into(), vec![0])
		]
	);
	Ok(())
}
