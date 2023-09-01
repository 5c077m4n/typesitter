use std::iter::{self, Peekable};

use log::{debug, error};

use super::{
	detector::all_tokens,
	token::{
		punctuation::Punctuation,
		token_variance::{Span, Token, TokenType},
	},
};

pub fn scan(input: &[u8], extra: Option<String>) -> Box<Peekable<impl Iterator<Item = Token<'_>>>> {
	Box::new(
		iter::from_fn({
			let mut input = Span::new_extra(input, extra);

			move || match all_tokens(input.to_owned()) {
				Ok((
					_,
					Token {
						value: TokenType::Empty,
						..
					},
				)) => None,
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
				value: TokenType::Punctuation(punc),
				..
			} if punc == Punctuation::Space
				|| punc == Punctuation::Tab
				|| punc == Punctuation::EOL =>
			{
				None
			}
			other => Some(other),
		})
		.peekable(),
	)
}
