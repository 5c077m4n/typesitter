use super::instr::{Instr, Pointer, Program};
use anyhow::{anyhow, Result};
use ast::types::{literal::Literal, node::Node};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct CodeGen {
	program: Program,
	variables: BTreeMap<String, Pointer>,
}
impl CodeGen {
	pub fn run(&mut self, tree: &Node) -> Result<Program> {
		match tree {
			Node::Block(instr_list) => {
				for instr in instr_list {
					self.run(instr)?;
				}
			}
			Node::VarDecl(var_decl) => match &*var_decl.value {
				Node::Literal(Literal::Number(n)) => {
					let var_name = var_decl.get_name().join(".");

					self.variables.insert(var_name, self.program.len());
					self.program.push(Instr::Push(*n));
				}
				other => unimplemented!("{:?} type is not supported yet", &other),
			},
			Node::VarCall(var_name) => {
				let var_name = String::from_utf8(var_name.to_vec())?;
				let pointer = self.variables.get(&var_name).ok_or_else(|| {
					anyhow!("Could not find the requested variable `{}`", &var_name)
				})?;

				self.program.push(Instr::Get(*pointer));
			}
			other => unimplemented!("{:?}", other),
		}
		Ok(self.program.to_owned())
	}
}
