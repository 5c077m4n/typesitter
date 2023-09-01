use super::super::types::{
	literal::Literal,
	node::Node,
	type_annotation::TypeAnnotation,
	var_decl::{VarDecl, VarType},
};
use anyhow::{bail, Result};
use lexer::token::{
	literal::Literal as TokenLiteral,
	punctuation::Punctuation,
	token_variance::{Token, TokenType},
};
use std::iter::Peekable;

pub fn parse_fn_decl_input_list<'a>(
	token_iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> Result<Vec<VarDecl<'a>>> {
	let mut input_token_index: usize = 0;
	let mut params: Vec<VarDecl> = Vec::new();

	while let Some(Token { value, position }) = token_iter.next() {
		match value {
			TokenType::Punctuation(Punctuation::BracketClose) => break,
			TokenType::Punctuation(Punctuation::Comma) => {
				if input_token_index == 0 {
					bail!(
						"Shouldn't put a comma as the first char in the fn input @ {:?}",
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
					name: vec![param_name],
					type_annotation: None,
					value: Box::new(Node::Literal(Literal::Undefined)),
				};
				params.push(param_dec);
			}
			TokenType::Punctuation(Punctuation::Colon) => {
				if input_token_index == 0 {
					bail!(
						"Shouldn't put a colon as the first char in the fn input @ {:?}",
						&position
					);
				}

				match token_iter.next() {
					Some(Token {
						value: TokenType::Identifier(fn_return_type),
						..
					}) => {
						params.last_mut().unwrap().type_annotation =
							Some(TypeAnnotation::try_from(fn_return_type)?);
					}
					other => {
						bail!("Wasn't expecting {:?} @ {:?}", &other, &position);
					}
				}
			}
			other => {
				bail!("Wasn't expecting {:?} @ {:?}", &other, &position);
			}
		}
	}

	Ok(params)
}

pub fn parse_fn_call_input_list<'a>(
	token_iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> Result<Vec<VarDecl<'a>>> {
	let mut input_token_index: usize = 0;
	let mut params: Vec<VarDecl> = Vec::new();

	for Token { value, position } in token_iter.by_ref() {
		match value {
			TokenType::Punctuation(Punctuation::BracketClose) => break,
			TokenType::Punctuation(Punctuation::Comma) => {
				if input_token_index == 0 {
					bail!(
						"Shouldn't put a comma as the first char in the fn input @ {:?}",
						&position
					);
				} else {
					input_token_index += 1;
				}
			}
			TokenType::Literal(lit) => {
				input_token_index += 1;

				match lit {
					TokenLiteral::Number(n) => {
						let param_dec = VarDecl {
							var_type: VarType::Let,
							name: vec![],
							type_annotation: Some(TypeAnnotation::Number),
							value: Box::new(Node::Literal(Literal::Number(n))),
						};
						params.push(param_dec);
					}
					other => {
						todo!("Non-number types aren't supported yet ({:?})", &other);
					}
				}
			}
			TokenType::Identifier(param_name) => {
				input_token_index += 1;

				let param_dec = VarDecl {
					var_type: VarType::Let,
					name: vec![param_name],
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
