use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use ast::types::{literal::Literal, node::Node};

use super::instr::{Instr, Pointer, Program};

#[derive(Default)]
pub struct CodeGen {
	program: Program,
	variables: BTreeMap<String, Pointer>,
	functions: BTreeMap<String, Pointer>,
}
impl CodeGen {
	fn get_var(&self, var_name: &str) -> Result<&Pointer> {
		self.variables
			.get(var_name)
			.ok_or_else(|| anyhow!("Could not find the requested variable `{}`", &var_name))
	}

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
				let pointer = self.get_var(var_name)?;
				self.program.push(Instr::Get(*pointer));
			}
			Node::FnCall(fn_call) => {
				let fn_name = fn_call.get_name().join(".");
				let params: Vec<Pointer> = fn_call
					.params
					.iter()
					.map(|param| {
						let name = param.get_name().join(".");
						*self.get_var(&name).unwrap()
					})
					.collect();

				if self.functions.contains_key(&fn_name) {
					unimplemented!("Function creation isn't supported yet");
				} else {
					self.program.push(Instr::CallBuiltin(fn_name, params));
				}
			}
			other => unimplemented!("{:?}", other),
		}
		Ok(self.program.to_owned())
	}
}
