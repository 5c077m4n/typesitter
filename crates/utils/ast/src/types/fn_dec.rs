use super::{node::Node, type_annotation::TypeAnnotation, var_dec::VarDecl};

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum FnType {
	Arrow,
	Classic,
}

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct FnDecl<'f> {
	pub fn_type: FnType,
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	pub name: Option<Vec<&'f [u8]>>,
	pub input_params: Vec<VarDecl<'f>>,
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	pub return_type: Option<TypeAnnotation<'f>>,
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	pub body: Box<Node<'f>>,
}
