use super::{node::Node, type_annotation::TypeAnnotation};
use anyhow::{bail, Error};
use lexer::token::keyword::Keyword;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VarDecl<'v> {
	pub var_type: VarType,
	pub name: Vec<&'v str>,
	#[serde(borrow)]
	pub type_annotation: Option<TypeAnnotation<'v>>,
	pub value: Box<Node<'v>>,
}
