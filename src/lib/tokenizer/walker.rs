use std::iter;

use super::token::Token;

pub fn walk<'s>() -> impl Iterator<Item = Token<'s>> {
	iter::from_fn(|| None)
}
