use std::collections::BTreeMap;

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Literal<'l> {
	Undefined,
	Null,
	String(&'l str),
	Number(f64),
	Bool(bool),
	Array(Vec<Literal<'l>>),
	Object(BTreeMap<&'l str, Box<Literal<'l>>>),
}
impl Default for Literal<'_> {
	fn default() -> Self {
		Self::Undefined
	}
}
