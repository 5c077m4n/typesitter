use super::node::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BinOp<'b> {
	op: Operator,
	#[serde(borrow)]
	lhs: Box<Node<'b>>,
	#[serde(borrow)]
	rhs: Box<Node<'b>>,
}
