use super::super::types::{
	literal::Literal,
	node::Node,
	var_dec::{VarDecl, VarType},
};
use anyhow::{bail, Result};
use lexer::token::{
	punctuation::Punctuation,
	token_variance::{Token, TokenType},
};
use std::iter::Peekable;

pub fn parse_param_list<'a>(
	token_iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> Result<Vec<VarDecl<'a>>> {
	let mut input_token_index: usize = 0;
	let mut params: Vec<VarDecl> = Vec::new();

	while let Some(Token { value, position }) = token_iter.next() {
		match value {
			TokenType::Punctuation(Punctuation::BracketClose) => {
				break;
			}
			TokenType::Punctuation(Punctuation::Colon) => match token_iter.next() {
				Some(Token {
					value: TokenType::Identifier(fn_return_type),
					..
				}) => {
					if let Some(last_param) = params.last_mut() {
						last_param.type_annotation = Some(fn_return_type);
					}
				}
				other => {
					bail!("Wasn't expecting {:?} @ {:?}", &other, &position);
				}
			},
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
				let param_dec = VarDecl {
					var_type: VarType::Let,
					name: param_name,
					type_annotation: None,
					value: Box::new(Node::Literal(Literal::Undefined)),
				};
				params.push(param_dec);
			}
			other => {
				bail!("Wasn't expecting {:?} @ {:?}", &other, &position);
			}
		}
	}

	Ok(params)
}
