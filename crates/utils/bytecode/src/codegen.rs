use super::instr::{Instr, Program};
use anyhow::Result;
use ast::types::{literal::Literal, node::Node, var_decl::VarDecl};

#[derive(Default)]
pub struct CodeGen {
	program: Program,
}
impl CodeGen {
	pub fn run(&mut self, tree: &Node) -> Result<Program> {
		match tree {
			Node::Block(instr_list) => {
				for instr in instr_list {
					self.run(instr)?;
				}
			}
			Node::VarDecl(VarDecl { value, .. }) => match &**value {
				Node::Literal(Literal::Number(n)) => self.program.push(Instr::Push(*n)),
				other => unimplemented!("{:?} type is not supported yet", other),
			},
			other => unimplemented!("{:?}", other),
		}
		Ok(self.program.to_owned())
	}
}
