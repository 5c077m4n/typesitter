use super::instr::{Instr, Program};
use anyhow::Result;
use ast::types::{literal::Literal, node::Node, var_decl::VarDecl};

fn convert<'c>(tree: &'c Node, prog: &'c mut Vec<Instr>) -> Result<&'c Vec<Instr>> {
	match tree {
		Node::Block(instr_list) => {
			for instr in instr_list {
				convert(instr, prog)?;
			}
		}
		Node::VarDecl(VarDecl { value, .. }) => match &**value {
			Node::Literal(Literal::Number(n)) => prog.push(Instr::Push(*n)),
			other => unimplemented!("{:?} type is not supported yet", other),
		},
		other => unimplemented!("{:?}", other),
	}
	Ok(prog)
}

pub fn codegen(tree: &Node) -> Result<Program> {
	let mut program: Vec<Instr> = Vec::new();
	let program = convert(tree, &mut program)?;

	Ok(program.to_owned())
}
