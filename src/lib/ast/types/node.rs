use super::{
	array::Array,
	bin_op::BinOp,
	fn_call::FnCall,
	fn_dec::FnDec,
	literal::Literal,
	object::Object,
	r#if::If,
	unary_op::UnaryOp,
	var_dec::VarDecl,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Node<'n> {
	Block(Vec<Box<Node<'n>>>),
	Literal(Literal<'n>),
	Object(Object<'n>),
	Array(Array<'n>),
	BinOp(BinOp<'n>),
	UnaryOp(UnaryOp<'n>),
	If(If<'n>),
	VarDecl(VarDecl<'n>),
	FnDec(FnDec<'n>),
	FnCall(FnCall<'n>),
	Return(Box<Node<'n>>),
}
