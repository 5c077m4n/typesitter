use anyhow::{bail, Result};
use ast::types::{literal::Literal, node::Node, type_annotation::TypeAnnotation, var_dec::VarDecl};
use inkwell::{builder::Builder, context::Context, module::Module, values::BasicValueEnum};

pub struct Compiler<'c, 'ctx> {
	context: &'ctx Context,
	module: &'c Module<'ctx>,
	builder: &'c Builder<'ctx>,
}
impl<'c, 'ctx> Compiler<'c, 'ctx> {
	fn compile_var(&self, var_decl: &VarDecl) -> Result<BasicValueEnum<'ctx>> {
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
			None | Some(TypeAnnotation::Any) => todo!("Support `any` type"),
			_ => unimplemented!("Compile var decl"),
		}
	}

	fn compile_instr(&self, node: &Node) -> Result<()> {
		match node {
			Node::VarDecl(var_decl) => self.compile_var(var_decl)?,
			_ => unimplemented!("Compile instruction"),
		};
		Ok(())
	}

	pub fn compile(&self, tree: &Node) -> Result<()> {
		self.compile_instr(tree)
	}
}
