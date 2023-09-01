use super::{
	detector::all_tokens,
	token::{
		punctuation::Punctuation,
		token_variance::{Span, Token, TokenType},
	},
};
use log::{debug, error};
use std::iter;

pub fn scan(input: &str, extra: Option<String>) -> impl Iterator<Item = Token<'_>> {
	iter::from_fn({
		let mut input = Span::new_extra(input, extra);

		move || match all_tokens(input.to_owned()) {
			Ok((_, Token { value, .. })) if value == TokenType::Empty => None,
			Ok((tail, token)) => {
				input = tail;
				debug!("{:#?}", &token);

				Some(token)
			}
			Err(error) => {
				error!("{:#?}", &error);
				None
			}
		}
	})
	.filter_map(|token| match token {
		Token {
			value: TokenType::Generic(frag),
			..
		} if frag.trim() == "" => None,
		Token {
			value: TokenType::Punctuation(Punctuation::Semicolon),
			..
		} => None,
		other => Some(other),
	})
}
