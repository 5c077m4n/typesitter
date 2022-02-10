pub mod types;

use self::types::{
	node::Node,
	var_dec::{VarDec, VarType},
};
use super::lexer::token::{keyword::Keyword, Token, TokenType};

pub struct AST<'a> {
	ast: Box<Node<'a>>,
}
impl<'a> AST<'a> {
	pub fn new() -> Self {
		Self {
			ast: Box::new(Node::Block(Box::new(Vec::new()))),
		}
	}
	pub fn parse(token_iter: impl Iterator<Item = Token<'a>>) {
		let root = Self::new();

		for token in token_iter {
			match token.value {
				TokenType::Keyword(Keyword::Const) => {
					if let Node::Block(expr_vec) = *root.ast {
						let const_node = Node::VarDec(Box::new(VarDec {
							var_type: VarType::Const,
							name: todo!(),
							type_annotation: None,
							value: todo!(),
						}));
						expr_vec.push(const_node);
					}
				}
				_ => unimplemented!(),
			}
		}
	}
}
