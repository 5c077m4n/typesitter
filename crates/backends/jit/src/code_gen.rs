#![allow(dead_code)]

use inkwell::{
	builder::Builder,
	context::Context,
	execution_engine::ExecutionEngine,
	module::Module,
};

pub struct CodeGen<'ctx> {
	context: &'ctx Context,
	module: Module<'ctx>,
	builder: Builder<'ctx>,
	execution_engine: ExecutionEngine<'ctx>,
}
