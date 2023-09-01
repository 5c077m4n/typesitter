use super::node::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Operator {
	Typeof,
	Plus,
	Minus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnaryOp<'u> {
	op: Operator,
	#[serde(borrow)]
	value: Box<Node<'u>>,
}
