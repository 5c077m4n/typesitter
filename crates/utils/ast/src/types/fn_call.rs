use super::var_decl::VarDecl;

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct FnCall<'f> {
	pub fn_name: Vec<&'f [u8]>,
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	pub params: Vec<VarDecl<'f>>,
}

impl FnCall<'_> {
	pub fn get_name(&self) -> Vec<&str> {
		self.fn_name
			.iter()
			.map(|v| std::str::from_utf8(v).unwrap())
			.collect::<Vec<_>>()
	}
}
