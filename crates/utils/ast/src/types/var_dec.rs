use super::{node::Node, type_annotation::TypeAnnotation};
use anyhow::{bail, Error};
use lexer::token::keyword::Keyword;

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
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
	pub name: Vec<&'v [u8]>,
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	pub type_annotation: Option<TypeAnnotation<'v>>,
	pub value: Box<Node<'v>>,
}
