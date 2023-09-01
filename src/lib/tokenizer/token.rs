use super::super::ast::{keyword::Keyword, literal::Literal, punctuation::Punctuation};
use nom_locate::LocatedSpan;

pub type Span<'s> = LocatedSpan<&'s str, Option<String>>;

#[derive(Debug, PartialEq)]
pub enum TokenType<'t> {
	Generic(&'t str),
	Keyword(Keyword),
	Punctuation(Punctuation),
	Literal(Literal<'t>),
}
#[derive(Debug, PartialEq)]
pub struct Token<'t> {
	pub position: Span<'t>,
	pub value: TokenType<'t>,
}
