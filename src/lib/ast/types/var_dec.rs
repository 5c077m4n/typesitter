use super::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum VarType {
	Const,
	Let,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarDec<'v> {
	pub var_type: VarType,
	pub name: &'v str,
	pub type_annotation: Option<&'v str>,
	pub value: Box<Node<'v>>,
}
