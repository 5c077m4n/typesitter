use super::instr::compile_instr;
use anyhow::Result;
use ast::types::node::Node;
use inkwell::{context::Context, OptimizationLevel};

pub fn compile(tree: Box<Node>) -> Result<(), Box<dyn std::error::Error>> {
	let context = Context::create();
	let builder = context.create_builder();
	let module = context.create_module("index");
	let _ee = module.create_jit_execution_engine(OptimizationLevel::None)?;

	compile_instr(&builder, &module, tree)?;

	Ok(())
}
