use super::{
	array::Array,
	bin_op::BinOp,
	fn_call::FnCall,
	fn_dec::FnDec,
	literal::Literal,
	object::Object,
	r#if::If,
	unary_op::UnaryOp,
	var_dec::VarDec,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Node<'n> {
	Block(Vec<Node<'n>>),
	Literal(Box<Literal<'n>>),
	Object(Box<Object<'n>>),
	Array(Box<Array<'n>>),
	BinOp(Box<BinOp<'n>>),
	UnaryOp(Box<UnaryOp<'n>>),
	If(Box<If<'n>>),
	VarDec(Box<VarDec<'n>>),
	FnDec(Box<FnDec<'n>>),
	FnCall(Box<FnCall<'n>>),
	Return(Box<Node<'n>>),
}
