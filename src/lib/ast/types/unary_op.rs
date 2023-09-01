use super::node::Node;

#[derive(Debug)]
pub enum Operator {
	Typeof,
}

#[derive(Debug)]
pub struct UnaryOp<'u> {
	op: Operator,
	value: &'u Node<'u>,
}
