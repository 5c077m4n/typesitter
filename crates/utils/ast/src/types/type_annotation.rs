use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TypeAnnotation<'t> {
	Unknown,
	Never,
	Void,
	Any,
	Number,
	String,
	Record,
	Array,
	Generic(&'t str),
}
impl<'t> TryFrom<&'t str> for TypeAnnotation<'t> {
	type Error = Error;

	fn try_from(value: &'t str) -> Result<Self, Self::Error> {
		match value {
			"unknown" => Ok(Self::Unknown),
			"never" => Ok(Self::Never),
			"void" => Ok(Self::Void),
			"any" => Ok(Self::Any),
			"number" => Ok(Self::Number),
			"string" => Ok(Self::String),
			"Record" => Ok(Self::Record),
			"Array" => Ok(Self::Array),
			"[]" => Ok(Self::Array),
			other => Ok(Self::Generic(other)),
		}
	}
}
