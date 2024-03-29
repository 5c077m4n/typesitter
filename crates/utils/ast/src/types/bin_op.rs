use super::node::Node;

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
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

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct BinOp<'b> {
	pub op: Operator,
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	pub lhs: Box<Node<'b>>,
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	pub rhs: Box<Node<'b>>,
}
