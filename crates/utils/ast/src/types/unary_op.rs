use super::node::Node;

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
	Typeof,
	Plus,
	Minus,
}

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct UnaryOp<'u> {
	op: Operator,
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	value: Box<Node<'u>>,
}
