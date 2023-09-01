use super::lexer::token::Token;

pub struct Statement<'s> {
	token_list: &'s [&'s Token<'s>],
}
pub struct Block<'b> {
	statement_list: &'b [&'b Statement<'b>],
}

pub enum Node<'n> {
	Block(Block<'n>),
	If(&'n Node<'n>, &'n Node<'n>, Option<&'n Node<'n>>),
	BinOp(&'n Node<'n>, &'n Node<'n>),
	UnaryOp(&'n Node<'n>),
	Return(&'n Node<'n>),
	FnInit(&'n Node<'n>),
}
