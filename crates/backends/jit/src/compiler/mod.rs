use anyhow::Result;
use ast::types::{node::Node, var_dec::VarDecl};
use inkwell::{builder::Builder, context::Context, module::Module, OptimizationLevel};

fn compile_instr<'c, 'ctx>(
	builder: &'c Builder<'ctx>,
	module: &'c Module<'ctx>,
	node: Box<Node>,
) -> Result<(), Box<dyn std::error::Error>> {
	let context = module.get_context();

	match *node {
		Node::VarDecl(VarDecl {
			var_type,
			name,
			type_annotation,
			value,
		}) => {}
		_ => unimplemented!("Compile instruction"),
	}

	Ok(())
}

pub fn compile(tree: Box<Node>) -> Result<(), Box<dyn std::error::Error>> {
	let context = Context::create();
	let builder = context.create_builder();
	let module = context.create_module("index");
	let _ee = module.create_jit_execution_engine(OptimizationLevel::None)?;

	compile_instr(&builder, &module, tree)?;

	Ok(())
}
