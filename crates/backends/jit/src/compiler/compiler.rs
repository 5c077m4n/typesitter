use anyhow::{bail, Result};
use ast::types::{literal::Literal, node::Node, type_annotation::TypeAnnotation, var_dec::VarDecl};
use inkwell::{builder::Builder, context::Context, module::Module, values::BasicValueEnum};

pub struct Compiler<'c, 'ctx> {
	context: &'ctx Context,
	module: &'c Module<'ctx>,
	builder: &'c Builder<'ctx>,
}
impl<'c, 'ctx> Compiler<'c, 'ctx> {
	pub fn compile_var(&'ctx self, var_decl: &VarDecl) -> Result<BasicValueEnum<'ctx>> {
		match var_decl.type_annotation {
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
			None | Some(TypeAnnotation::Any) => todo!("Support `Any` type"),
			_ => unimplemented!("Compile var decl"),
		}
	}

	pub fn compile_instr(&'ctx self, node: Box<Node>) -> Result<()> {
		let _instr = match *node {
			Node::VarDecl(var_decl) => self.compile_var(&var_decl)?,
			_ => unimplemented!("Compile instruction"),
		};

		Ok(())
	}

	pub fn compile(&'ctx self, tree: Box<Node>) -> Result<()> {
		self.compile_instr(tree)?;
		Ok(())
	}
}
