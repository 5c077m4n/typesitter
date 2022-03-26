use super::{keyword::Keyword, literal::Literal, punctuation::Punctuation};
use nom_locate::LocatedSpan;

pub type Span<'s> = LocatedSpan<&'s str, Option<String>>;

#[derive(Debug, PartialEq)]
pub enum TokenType<'t> {
	// TODO: find out why this is needed & remove this
	Empty,
	Generic(&'t str),
	Identifier(&'t str),
	Keyword(Keyword),
	Punctuation(Punctuation),
	Literal(Literal<'t>),
}
#[derive(Debug, PartialEq)]
pub struct Token<'t> {
	pub position: Span<'t>,
	pub value: TokenType<'t>,
}
