use nom_locate::LocatedSpan;

use super::{keyword::Keyword, literal::Literal, punctuation::Punctuation};

pub type Span<'s> = LocatedSpan<&'s [u8], Option<String>>;

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq)]
pub enum TokenType<'t> {
	// TODO: find out why this is needed & remove this
	Empty,
	Generic(&'t [u8]),
	Identifier(&'t [u8]),
	Keyword(Keyword),
	Punctuation(Punctuation),
	Literal(Literal<'t>),
}

#[derive(Debug, PartialEq)]
pub struct Token<'t> {
	pub value: TokenType<'t>,
	pub position: Span<'t>,
}

#[cfg(feature = "js_bind")]
impl serde::Serialize for Token<'_> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::SerializeStruct;

		let mut state = serializer.serialize_struct("Token", 6)?;

		state.skip_field("position")?;
		state.serialize_field("fragment", &self.position.fragment())?;
		state.serialize_field("offset", &self.position.location_offset())?;
		state.serialize_field("line", &self.position.location_line())?;
		state.serialize_field("column", &self.position.get_column())?;
		state.serialize_field("value", &self.value)?;

		state.end()
	}
}
