use super::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
	Eq,
	EqEq,
	NeqEq,
	EqEqEq,
	NeqEqEq,
	Lt,
	Lte,
	Gt,
	Gte,
	Plus,
	Minus,
	Module,
	Div,
	Mul,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinOp<'u> {
	op: Operator,
	lhs: Box<Node<'u>>,
	rhs: Box<Node<'u>>,
}
