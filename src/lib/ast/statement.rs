use super::super::lexer::token::Token;

pub struct Statement<'s> {
	token_list: &'s [&'s Token<'s>],
}
