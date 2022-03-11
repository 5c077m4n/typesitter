use super::{
	super::types::{fn_call::FnCall, node::Node},
	param_list::parse_param_list,
};
use anyhow::Result;
use lexer::token::{
	punctuation::Punctuation,
	token_variance::{Token, TokenType},
};
use log::error;
use std::iter::Peekable;

pub fn ident_parse<'a>(
	initial_ident: &'a str,
	token_iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> Result<Vec<Node<'a>>> {
	let mut expr_list: Vec<Node<'a>> = Vec::new();

	while let Some(Token { value, position }) = token_iter.next() {
		match value {
			TokenType::Punctuation(Punctuation::BracketOpen) => {
				let params = parse_param_list(token_iter)?;
				let fn_call_node = Node::FnCall(FnCall {
					fn_name: initial_ident,
					params,
				});
				expr_list.push(fn_call_node);
			}
			TokenType::Punctuation(Punctuation::Dot) => {
				todo!("Object fields lookup");
			}
			TokenType::Punctuation(Punctuation::Equal) => {
				todo!("Param assignment");
			}
			TokenType::Punctuation(Punctuation::Semicolon) => break,
			other => {
				error!("{:?} @ {:?}", &other, &position);
				unimplemented!("Identifier {:?} @ {:?}", &other, &position);
			}
		}
	}

	Ok(expr_list)
}
