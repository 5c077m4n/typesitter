use super::node::Node;

#[derive(Debug)]
pub enum VarType {
	Const,
	Let,
}

#[derive(Debug)]
pub struct VarDec<'v> {
	pub var_type: VarType,
	pub name: &'v str,
	pub type_annotation: Option<&'v str>,
	pub value: &'v Node<'v>,
}
