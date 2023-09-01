#![allow(dead_code)]

use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal<'a> {
	String(&'a str),
	Number(f64),
	Bool(bool),
	Null,
	Undefined,

	Array(&'a Vec<&'a Literal<'a>>),
	Object(BTreeMap<&'a str, &'a Literal<'a>>),
}
pub struct Var<'a> {
	name: &'a str,
	is_const: bool,
	value: &'a Literal<'a>,
}

impl Default for Var<'_> {
	fn default() -> Self {
		Self {
			name: "",
			is_const: false,
			value: &Literal::Undefined,
		}
	}
}
