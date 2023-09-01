use log::error;
use std::iter;

use super::{
	token::{Span, Token},
	tokenizer::all_tokens,
};

pub fn scan(input: &str, extra: Option<String>) -> impl Iterator<Item = Token<'_>> {
	let mut input = Span::new_extra(input, extra);

	iter::from_fn(move || match all_tokens(input.clone()) {
		Ok((tail, token)) => {
			input = tail;
			Some(token)
		}
		Err(error) => {
			error!("{error:#?}");
			None
		}
	})
}
