use super::var_dec::VarDecl;

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct FnCall<'f> {
	pub fn_name: Vec<&'f [u8]>,
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	pub params: Vec<VarDecl<'f>>,
}
