use super::node::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct If<'i> {
	#[serde(borrow)]
	pub expr: Box<Node<'i>>,
	#[serde(borrow)]
	pub block: Box<Node<'i>>,
	pub else_block: Option<Box<Node<'i>>>,
}
