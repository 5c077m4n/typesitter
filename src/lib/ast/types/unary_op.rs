use super::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
	Typeof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryOp<'u> {
	op: Operator,
	value: &'u Node<'u>,
}
