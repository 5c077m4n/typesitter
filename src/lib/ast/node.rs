use super::{
	array::Array, bin_op::BinOp, fn_call::FnCall, fn_dec::FnDec, literal::Literal, object::Object,
	r#if::If, unary_op::UnaryOp,
};

#[derive(Debug)]
pub enum Node<'n> {
	Literal(&'n Literal<'n>),
	Object(&'n Object<'n>),
	Array(&'n Array<'n>),
	BinOp(&'n BinOp<'n>),
	UnaryOp(&'n UnaryOp<'n>),
	If(&'n If<'n>),
	FnDec(&'n FnDec<'n>),
	FnCall(&'n FnCall<'n>),
	Return(&'n Node<'n>),
}
