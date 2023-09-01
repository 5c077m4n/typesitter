use anyhow::{bail, Result};
use ast::types::{
	fn_dec::FnDecl,
	literal::Literal,
	node::Node,
	type_annotation::TypeAnnotation,
	var_dec::VarDecl,
};
use inkwell::{
	builder::Builder,
	context::Context,
	module::Module,
	types::AnyTypeEnum,
	values::{BasicValueEnum, FunctionValue},
};

pub struct Compiler<'c, 'ctx> {
	context: &'ctx Context,
	module: &'c Module<'ctx>,
	builder: &'c Builder<'ctx>,
}
impl<'c, 'ctx> Compiler<'c, 'ctx> {
	fn compile_fn(&self, fn_decl: &FnDecl) -> Result<FunctionValue<'ctx>> {
		let fn_return_type = match &fn_decl.return_type {
			Some(TypeAnnotation::Number) => AnyTypeEnum::FloatType(self.context.f64_type()),
			None | Some(TypeAnnotation::Void) => AnyTypeEnum::VoidType(self.context.void_type()),
			other => todo!("Support other function types ({:?})", other),
		};
		let fn_type = match fn_return_type {
			AnyTypeEnum::FloatType(ret_type) => ret_type.fn_type(&[], false),
			AnyTypeEnum::VoidType(ret_type) => ret_type.fn_type(&[], false),
			other => todo!("Support {:?} type", &other),
		};

		let func_val = self.module.add_function(
			&fn_decl
				.name
				.as_ref()
				.map(|name_list| name_list.join("."))
				.unwrap_or_else(|| "".to_string()),
			fn_type,
			None,
		);
		let basic_block = self.context.append_basic_block(func_val, "entry");
		self.builder.position_at_end(basic_block);

		todo!(
			"continue the `compile_instr` function and then continue this function (`compile_fn`)"
		);

		Ok(func_val)
	}

	fn compile_var_decl(&self, var_decl: &VarDecl) -> Result<BasicValueEnum<'ctx>> {
		match &var_decl.type_annotation {
			Some(TypeAnnotation::String) => match var_decl.value.as_ref() {
				Node::Literal(Literal::String(s)) => {
					let string_const = self.context.const_string(s.as_bytes(), false);
					Ok(BasicValueEnum::VectorValue(string_const))
				}
				other => bail!(
					"The var {} is annotated as a {:?} and parsed as a {:?}",
					var_decl.name.join("."),
					var_decl.type_annotation,
					other
				),
			},
			Some(TypeAnnotation::Number) => match var_decl.value.as_ref() {
				Node::Literal(Literal::Number(n)) => {
					let f64_type = self.context.f64_type();
					let f64_value = f64_type.const_float(*n);

					Ok(BasicValueEnum::FloatValue(f64_value))
				}
				other => bail!(
					"The var {} is annotated as a {:?} and parsed as a {:?}",
					var_decl.name.join("."),
					var_decl.type_annotation,
					other
				),
			},
			None | Some(TypeAnnotation::Any) => todo!("Support `any` type"),
			other => unimplemented!("Compile var decl for {:?}", other),
		}
	}

	fn compile_instr(&self, node: &Node) -> Result<()> {
		match node {
			Node::VarDecl(var_decl) => {
				self.compile_var_decl(var_decl)?;
			}
			Node::FnDecl(fn_decl) => {
				self.compile_fn(fn_decl)?;
			}
			other => unimplemented!("Compile instruction for {:?}", other),
		}
		Ok(())
	}

	pub fn compile(&self, tree: &Node) -> Result<()> {
		self.compile_instr(tree)
	}
}
