use std::collections::BTreeMap;

use anyhow::{bail, Error};
use lexer::token::literal::Literal as TokenLiteral;

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Literal<'l> {
	#[default]
	Undefined,
	Null,
	String(&'l [u8]),
	Number(f64),
	Bool(bool),
	Array(Vec<Literal<'l>>),
	Object(BTreeMap<&'l str, Box<Literal<'l>>>),
}

impl<'l> TryFrom<TokenLiteral<'l>> for Literal<'l> {
	type Error = Error;

	fn try_from(lit: TokenLiteral<'l>) -> Result<Self, Self::Error> {
		match lit {
			TokenLiteral::Undefined => Ok(Self::Undefined),
			TokenLiteral::Null => Ok(Self::Null),
			TokenLiteral::String(s) => Ok(Self::String(s)),
			TokenLiteral::Number(n) => Ok(Self::Number(n)),
			TokenLiteral::Bool(b) => Ok(Self::Bool(b)),
			other => bail!("{:?} is not parsable to `Literal`", &other),
		}
	}
}
