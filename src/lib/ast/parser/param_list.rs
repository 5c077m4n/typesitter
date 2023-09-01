use super::super::super::lexer::token::{
	punctuation::Punctuation,
	token_variance::{Token, TokenType},
};
use anyhow::{bail, Result};
use log::error;

pub fn parse_param_list<'a>(
	mut token_iter: impl Iterator<Item = Token<'a>>,
) -> Result<Vec<&'a str>> {
	let mut input_token_index: usize = 0;
	let mut params: Vec<&str> = Vec::new();

	for Token { value, position } in token_iter.by_ref() {
		match value {
			TokenType::Punctuation(Punctuation::BracketClose) => {
				break;
			}
			TokenType::Punctuation(Punctuation::Comma) => {
				if input_token_index == 0 {
					bail!(
						"Shouldn't put a comma as the first char in the fn input @ {}",
						&position
					);
				} else {
					input_token_index += 1;
				}
			}
			TokenType::Identifier(param_name) => {
				input_token_index += 1;
				params.push(param_name);
			}
			other => {
				error!("{:?}", &other);
				unimplemented!();
			}
		}
	}

	Ok(params)
}
