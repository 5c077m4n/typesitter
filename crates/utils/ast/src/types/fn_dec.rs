use super::{node::Node, type_annotation::TypeAnnotation, var_dec::VarDecl};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FnType {
	Arrow,
	Classic,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FnDecl<'f> {
	pub fn_type: FnType,
	#[serde(borrow)]
	pub name: Option<Vec<&'f str>>,
	pub input_params: Vec<VarDecl<'f>>,
	#[serde(borrow)]
	pub return_type: Option<TypeAnnotation<'f>>,
	#[serde(borrow)]
	pub body: Box<Node<'f>>,
}
