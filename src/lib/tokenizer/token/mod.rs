#![allow(dead_code)]

use nom_locate::LocatedSpan;

pub mod keyword;
pub mod literal;
pub mod punctuation;

pub type Span<'s> = LocatedSpan<&'s str, Option<String>>;

#[derive(Debug, PartialEq)]
pub enum TokenType<'t> {
	// TODO: find out why this is needed & remove this
	Empty,
	Generic(&'t str),
	Identifier(&'t str),
	Keyword(keyword::Keyword),
	Punctuation(punctuation::Punctuation),
	Literal(literal::Literal<'t>),
}
#[derive(Debug, PartialEq)]
pub struct Token<'t> {
	pub position: Span<'t>,
	pub value: TokenType<'t>,
}
