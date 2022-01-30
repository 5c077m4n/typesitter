use super::block::Block;

pub enum Node<'n> {
	Block(Block<'n>),
	If(&'n Node<'n>, &'n Node<'n>, Option<&'n Node<'n>>),
	BinOp(&'n Node<'n>, &'n Node<'n>),
	UnaryOp(&'n Node<'n>),
	Return(&'n Node<'n>),
	FnInit(&'n Node<'n>),
}
