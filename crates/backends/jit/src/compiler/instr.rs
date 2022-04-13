use super::var::compile_var;
use ast::types::node::Node;
use inkwell::{builder::Builder, module::Module};

pub fn compile_instr<'c, 'ctx>(
	builder: &'c Builder<'ctx>,
	module: &'c Module<'ctx>,
	node: Box<Node>,
) -> Result<(), Box<dyn std::error::Error>> {
	match *node {
		Node::VarDecl(var_decl) => compile_var(builder, module, &var_decl),
		_ => unimplemented!("Compile instruction"),
	}

	Ok(())
}
