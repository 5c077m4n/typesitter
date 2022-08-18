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
impl<'t> From<&'t str> for TypeAnnotation<'t> {
	fn from(value: &'t str) -> Self {
		match value {
			"unknown" => Self::Unknown,
			"never" => Self::Never,
			"void" => Self::Void,
			"any" => Self::Any,
			"number" => Self::Number,
			"string" => Self::String,
			"Record" => Self::Record,
			"Array" => Self::Array,
			"[]" => Self::Array,
			other => Self::Generic(other.as_bytes()),
		}
	}
}
impl<'t> From<&'t [u8]> for TypeAnnotation<'t> {
	fn from(value: &'t [u8]) -> Self {
		match value {
			b"unknown" => Self::Unknown,
			b"never" => Self::Never,
			b"void" => Self::Void,
			b"any" => Self::Any,
			b"number" => Self::Number,
			b"string" => Self::String,
			b"Record" => Self::Record,
			b"Array" => Self::Array,
			b"[]" => Self::Array,
			other => Self::Generic(other),
		}
	}
}
