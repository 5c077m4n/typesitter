use super::node::Node;
use anyhow::{bail, Error};
use lexer::token::keyword::Keyword;

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

#[derive(Clone, Debug, PartialEq)]
pub struct VarDecl<'v> {
	pub var_type: VarType,
	pub name: Vec<&'v str>,
	pub type_annotation: Option<&'v str>,
	pub value: Box<Node<'v>>,
}
