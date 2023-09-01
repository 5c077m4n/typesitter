use super::instr::{Instr, Program};
use anyhow::Result;
use ast::types::node::Node;

pub fn codegen(_tree: &Node) -> Result<Program> {
	let prograpm: Vec<Instr> = Vec::new();
	Ok(prograpm)
}
