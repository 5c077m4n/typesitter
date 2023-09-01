use super::{
	detector::all_tokens,
	token::token_variance::{Span, Token, TokenType},
};
use log::{debug, error};
use std::iter::{self, Peekable};

pub fn scan(input: &str, extra: Option<String>) -> Box<Peekable<impl Iterator<Item = Token<'_>>>> {
	Box::new(
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
			other => Some(other),
		})
		.peekable(),
	)
}
