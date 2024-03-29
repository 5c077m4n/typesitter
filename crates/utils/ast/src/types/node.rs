use super::{
	array::Array,
	bin_op::BinOp,
	fn_call::FnCall,
	fn_dec::FnDecl,
	literal::Literal,
	object::Object,
	r#if::If,
	unary_op::UnaryOp,
	var_decl::VarDecl,
};

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Node<'n> {
	Block(Vec<Node<'n>>),
	Literal(Literal<'n>),
	Object(Object<'n>),
	Array(Array<'n>),
	BinOp(BinOp<'n>),
	UnaryOp(UnaryOp<'n>),
	If(If<'n>),
	VarDecl(VarDecl<'n>),
	VarCall(&'n str),
	FnDecl(FnDecl<'n>),
	FnCall(FnCall<'n>),
	Return(Box<Node<'n>>),
}
