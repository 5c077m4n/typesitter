use log::{debug, error};
use std::iter;

use super::{
	detector::all_tokens,
	token::{Span, Token, TokenType},
};

pub fn scan(input: &str, extra: Option<String>) -> impl Iterator<Item = Token<'_>> {
	let mut input = Span::new_extra(input, extra);

	iter::from_fn(move || match all_tokens(input.to_owned()) {
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
	})
}
