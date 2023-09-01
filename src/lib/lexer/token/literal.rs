#![allow(dead_code)]

use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal<'l> {
	Undefined,
	Null,
	String(&'l str),
	Number(f64),
	Bool(bool),
	Array(Vec<Box<Literal<'l>>>),
	Object(BTreeMap<&'l str, Box<Literal<'l>>>),
}
impl Default for Literal<'_> {
	fn default() -> Self {
		Self::Undefined
	}
}
