use super::{node::Node, var_dec::VarDecl};

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
	pub return_type: Option<&'f str>,
	pub body: Box<Node<'f>>,
}
