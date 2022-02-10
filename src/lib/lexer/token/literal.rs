#![allow(dead_code)]

use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal<'l> {
	Undefined,
	Null,
	String(&'l str),
	Number(f64),
	Bool(bool),
	Array(Box<Vec<&'l Literal<'l>>>),
	Object(Box<BTreeMap<&'l str, &'l Literal<'l>>>),
}
impl Default for Literal<'_> {
	fn default() -> Self {
		Self::Undefined
	}
}
