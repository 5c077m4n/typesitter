use super::var_dec::VarDec;

#[derive(Clone, Debug, PartialEq)]
pub struct FnCall<'f> {
	pub fn_name: &'f str,
	pub params: Vec<VarDec<'f>>,
}
