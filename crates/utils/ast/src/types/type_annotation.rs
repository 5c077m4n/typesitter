use anyhow::{Error, Result};

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeAnnotation<'t> {
	Unknown,
	Never,
	Void,
	Any,
	Number,
	String,
	Record,
	Array,
	Generic(&'t [u8]),
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
			other => Ok(Self::Generic(other.as_bytes())),
		}
	}
}
impl<'t> TryFrom<&'t [u8]> for TypeAnnotation<'t> {
	type Error = Error;

	fn try_from(value: &'t [u8]) -> Result<Self, Self::Error> {
		match value {
			b"unknown" => Ok(Self::Unknown),
			b"never" => Ok(Self::Never),
			b"void" => Ok(Self::Void),
			b"any" => Ok(Self::Any),
			b"number" => Ok(Self::Number),
			b"string" => Ok(Self::String),
			b"Record" => Ok(Self::Record),
			b"Array" => Ok(Self::Array),
			b"[]" => Ok(Self::Array),
			other => Ok(Self::Generic(other)),
		}
	}
}
