use super::var_dec::VarDecl;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FnCall<'f> {
	pub fn_name: Vec<&'f str>,
	#[serde(borrow)]
	pub params: Vec<VarDecl<'f>>,
}
