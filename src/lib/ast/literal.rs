use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal<'a> {
	Undefined,
	Null,
	String(&'a str),
	Number(f64),
	Bool(bool),
	Array(&'a Vec<&'a Literal<'a>>),
	Object(&'a BTreeMap<&'a str, &'a Literal<'a>>),
}
impl Default for Literal<'_> {
	fn default() -> Self {
		Self::Undefined
	}
}
