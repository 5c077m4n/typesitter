use super::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
	Typeof,
	Plus,
	Minus,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryOp<'u> {
	op: Operator,
	value: Box<Node<'u>>,
}
