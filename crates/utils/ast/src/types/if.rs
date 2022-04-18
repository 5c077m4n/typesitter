use super::node::Node;

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct If<'i> {
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	pub expr: Box<Node<'i>>,
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	pub block: Box<Node<'i>>,
	pub else_block: Option<Box<Node<'i>>>,
}
