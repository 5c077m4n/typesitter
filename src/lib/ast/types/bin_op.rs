use super::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
	Plus,
	Minus,
	Module,
	Div,
	Mul,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinOp<'u> {
	op: Operator,
	lhs: &'u Node<'u>,
	rhs: &'u Node<'u>,
}
