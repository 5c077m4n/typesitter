use super::{codegen::CodeGen, instr::Instr};
use anyhow::Result;
use ast::types::{
	literal::Literal,
	node::Node,
	var_decl::{VarDecl, VarType},
};

#[test]
fn empty_program() -> Result<()> {
	let tree = Node::Block(vec![Node::Block(vec![])]);
	let mut codegen = CodeGen::default();
	let prog = codegen.run(&tree)?;

	assert_eq!(prog, vec![]);
	Ok(())
}

#[test]
fn number_decl() -> Result<()> {
	let tree = Node::Block(vec![Node::VarDecl(VarDecl {
		var_type: VarType::Const,
		name: vec![b"param_1"],
		type_annotation: None,
		value: Box::new(Node::Literal(Literal::Number(1.))),
	})]);
	let mut codegen = CodeGen::default();
	let prog = codegen.run(&tree)?;

	assert_eq!(prog, vec![Instr::Push(1.)]);
	Ok(())
}
