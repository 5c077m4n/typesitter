use super::{node::Node, var_dec::VarDec};

#[derive(Debug)]
pub enum FnType {
	Arrow,
	Classic,
}
#[derive(Debug)]
pub struct FnDec<'f> {
	fn_type: FnType,
	name: Option<&'f str>,
	input_params: &'f [&'f VarDec<'f>],
	return_type: Option<&'f str>,
	body: &'f [&'f Node<'f>],
}
