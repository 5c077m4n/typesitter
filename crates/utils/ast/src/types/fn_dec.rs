use super::{node::Node, type_annotation::TypeAnnotation, var_dec::VarDecl};

#[derive(Clone, Debug, PartialEq)]
pub enum FnType {
	Arrow,
	Classic,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FnDec<'f> {
	pub fn_type: FnType,
	pub name: Option<Vec<&'f str>>,
	pub input_params: Vec<VarDecl<'f>>,
	pub return_type: Option<TypeAnnotation<'f>>,
	pub body: Box<Node<'f>>,
}
