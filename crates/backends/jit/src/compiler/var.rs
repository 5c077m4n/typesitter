use ast::types::{type_annotation::TypeAnnotation, var_dec::VarDecl};
use inkwell::{builder::Builder, module::Module};

pub fn compile_var<'c, 'ctx>(
	builder: &'c Builder<'ctx>,
	module: &'c Module<'ctx>,
	var_decl: &VarDecl,
) {
	let context = module.get_context();

	match var_decl.type_annotation {
		Some(TypeAnnotation::Number) => {}
		None | Some(TypeAnnotation::Any) => {}
		_ => unimplemented!("Compile var decl"),
	}
}
