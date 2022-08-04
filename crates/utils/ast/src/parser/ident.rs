use super::{
	super::types::{fn_call::FnCall, node::Node},
	param_list::parse_fn_call_input_list,
};
use anyhow::Result;
use lexer::token::{
	punctuation::Punctuation,
	token_variance::{Token, TokenType},
};
use log::error;
use std::iter::Peekable;

pub fn ident_parse<'a>(
	ident: &'a [u8],
	token_iter: &mut Peekable<impl Iterator<Item = Token<'a>>>,
) -> Result<Vec<Node<'a>>> {
	let mut expr_list: Vec<Node<'a>> = Vec::new();

	let mut ident_parts = vec![ident];
	while let Some(Token { value, .. }) = token_iter.peek() {
		match value {
			TokenType::Identifier(ident_part) => ident_parts.push(ident_part),
			TokenType::Punctuation(Punctuation::Dot) => {}
			_ => break,
		}
		token_iter.next();
	}

	while let Some(Token { value, position }) = token_iter.next() {
		match value {
			TokenType::Punctuation(Punctuation::Semicolon) => break,
			TokenType::Punctuation(Punctuation::BracketOpen) => {
				let fn_name = ident_parts.to_owned();
				let params = parse_fn_call_input_list(token_iter)?;

				let fn_call_node = Node::FnCall(FnCall { fn_name, params });
				expr_list.push(fn_call_node);
			}
			TokenType::Punctuation(Punctuation::Equal) => {
				todo!("Param assignment");
			}
			other => {
				error!("{:?} @ {:?}", &other, &position);
				unimplemented!("Identifier {:?} @ {:?}", &other, &position);
			}
		}
	}

	Ok(expr_list)
}
