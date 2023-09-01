use super::super::super::{
	ast::types::{
		literal::Literal,
		node::Node,
		var_dec::{VarDec, VarType},
	},
	lexer::token::{
		punctuation::Punctuation,
		token_variance::{Token, TokenType},
	},
};
use anyhow::{bail, Result};
use log::error;

pub fn parse_param_list<'a>(
	mut token_iter: impl Iterator<Item = Token<'a>>,
) -> Result<Vec<VarDec<'a>>> {
	let mut input_token_index: usize = 0;
	let mut params: Vec<VarDec> = Vec::new();

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
				let param_dec = VarDec {
					var_type: VarType::Let,
					name: param_name,
					type_annotation: None,
					value: Box::new(Node::Literal(Literal::Undefined)),
				};
				params.push(param_dec);
			}
			other => {
				error!("{:?}", &other);
				unimplemented!();
			}
		}
	}

	Ok(params)
}
