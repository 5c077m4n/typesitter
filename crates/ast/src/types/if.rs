use super::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub struct If<'i> {
	pub expr: Box<Node<'i>>,
	pub block: Box<Node<'i>>,
	pub else_block: Option<Box<Node<'i>>>,
}
