use super::node::Node;

#[derive(Debug)]
pub enum VarType {
	Const,
	Let,
}

#[derive(Debug)]
pub struct VarDec<'v> {
	var_type: VarType,
	name: &'v str,
	type_annotation: Option<&'v str>,
	value: &'v Node<'v>,
}
