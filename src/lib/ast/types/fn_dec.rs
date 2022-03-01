use super::{node::Node, var_dec::VarDec};

#[derive(Clone, Debug, PartialEq)]
pub enum FnType {
	Arrow,
	Classic,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FnDec<'f> {
	pub fn_type: FnType,
	pub name: Option<&'f str>,
	pub input_params: Vec<VarDec<'f>>,
	pub return_type: Option<&'f str>,
	pub body: Box<Node<'f>>,
}
