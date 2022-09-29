use anyhow::{bail, Error, Result};
use lexer::token::keyword::Keyword;

use super::{node::Node, type_annotation::TypeAnnotation};

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VarType {
	Const,
	Let,
}
impl TryFrom<Keyword> for VarType {
	type Error = Error;

	fn try_from(kw: Keyword) -> Result<Self, Self::Error> {
		match kw {
			Keyword::Const => Ok(Self::Const),
			Keyword::Let => Ok(Self::Let),
			other => bail!("{:?} is not parsable to `VarType`", &other),
		}
	}
}

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct VarDecl<'v> {
	pub var_type: VarType,
	pub name: Vec<&'v str>,
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	pub type_annotation: Option<TypeAnnotation<'v>>,
	pub value: Box<Node<'v>>,
}

impl VarDecl<'_> {
	pub fn get_name(&self) -> Vec<&str> {
		self.name.to_vec()
	}
}
