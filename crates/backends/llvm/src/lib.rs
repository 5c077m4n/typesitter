mod compiler;

use anyhow::Result;
use ast::types::node::Node;
use compiler::Compiler;
use inkwell::{context::Context, passes::PassManager, OptimizationLevel};

pub type ExternFn = unsafe extern "C" fn() -> f64;

pub fn run(tree: &Node) -> Result<()> {
	let context = &Context::create();
	let module = &context.create_module("index");
	let builder = &context.create_builder();

	let pass_manager = &PassManager::create(module);
	pass_manager.add_instruction_combining_pass();
	pass_manager.add_reassociate_pass();
	pass_manager.add_gvn_pass();
	pass_manager.add_cfg_simplification_pass();
	pass_manager.add_basic_alias_analysis_pass();
	pass_manager.add_promote_memory_to_register_pass();
	pass_manager.add_instruction_combining_pass();
	pass_manager.add_reassociate_pass();
	pass_manager.initialize();

	Compiler::compile(context, module, builder, pass_manager, tree)?;

	let exec_engine = module
		.create_jit_execution_engine(OptimizationLevel::Default)
		.unwrap();
	let compiled_fn = unsafe { exec_engine.get_function::<ExternFn>("main") }?;
	unsafe { compiled_fn.call() };

	Ok(())
}
