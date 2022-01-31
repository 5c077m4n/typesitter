use super::node::Node;

#[derive(Debug)]
pub enum Operator {
	Plus,
	Minus,
	Module,
	Div,
	Mul,
}

#[derive(Debug)]
pub struct BinOp<'u> {
	op: Operator,
	lhs: &'u Node<'u>,
	rhs: &'u Node<'u>,
}
