use super::var_dec::VarDecl;

#[derive(Clone, Debug, PartialEq)]
pub struct FnCall<'f> {
	pub fn_name: Vec<&'f str>,
	pub params: Vec<VarDecl<'f>>,
}
