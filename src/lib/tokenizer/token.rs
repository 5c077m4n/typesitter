use super::super::ast::{keyword::Keyword, literal::Literal, punctuation::Punctuation};
use nom_locate::LocatedSpan;

pub type Span<'s> = LocatedSpan<&'s str, Option<String>>;

#[derive(PartialEq, Eq)]
pub struct GenericToken<'s> {
	pub position: Span<'s>,
	pub token: &'s str,
}
#[derive(PartialEq)]
pub struct LiteralToken<'s> {
	pub position: Span<'s>,
	pub token: Literal<'s>,
}
#[derive(PartialEq, Eq)]
pub struct KeywordToken<'s> {
	pub position: Span<'s>,
	pub token: Keyword,
}
#[derive(PartialEq, Eq)]
pub struct PunctuationToken<'s> {
	pub position: Span<'s>,
	pub token: Punctuation,
}
