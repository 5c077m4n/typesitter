use log::{debug, error};
use std::iter;

use super::{
	token::{Span, Token, TokenType},
	tokenizer::all_tokens,
};

pub fn scan(input: &str, extra: Option<String>) -> impl Iterator<Item = Token<'_>> {
	let mut input = Span::new_extra(input, extra);

	iter::from_fn(move || match all_tokens(input.to_owned()) {
		Ok((tail, token)) => {
			input = tail;
			debug!("{:?}", &input);

			if let TokenType::Generic("") = token.value {
				None
			} else {
				Some(token)
			}
		}
		Err(error) => {
			error!("{error:#?}");
			None
		}
	})
}
