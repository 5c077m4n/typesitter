use super::{
	super::lexer::token::{
		keyword::Keyword, literal::Literal as TokenLiteral, punctuation::Punctuation, Token,
		TokenType,
	},
	types::{
		literal::Literal,
		node::Node,
		var_dec::{VarDec, VarType},
	},
};

pub fn parse<'a>(mut token_iter: impl Iterator<Item = Token<'a>>) -> Box<Vec<Node<'a>>> {
	let mut expr_list = Box::new(Vec::new());

	while let Some(Token { value, .. }) = token_iter.next() {
		match value {
			TokenType::Keyword(init_type @ (Keyword::Const | Keyword::Let)) => {
				if let Some(Token {
					value: TokenType::Generic(param_name),
					..
				}) = token_iter.next()
				{
					match token_iter.next() {
						Some(Token {
							value: TokenType::Punctuation(Punctuation::Colon),
							..
						}) => {
							if let Some(Token {
								value: TokenType::Generic(var_type),
								..
							}) = token_iter.next()
							{
								if let Some(Token {
									value: TokenType::Punctuation(Punctuation::Equal),
									..
								}) = token_iter.next()
								{
									if let Some(Token {
										value: TokenType::Literal(TokenLiteral::Number(n)),
										..
									}) = token_iter.next()
									{
										let init_node = Node::VarDec(Box::new(VarDec {
											var_type: if init_type == Keyword::Let {
												VarType::Let
											} else {
												VarType::Const
											},
											name: param_name,
											type_annotation: Some(var_type),
											value: Box::new(Node::Literal(Box::new(
												Literal::Number(n),
											))),
										}));

										expr_list.push(init_node);
									}
								}
							}
						}
						Some(Token {
							value: TokenType::Punctuation(Punctuation::Equal),
							..
						}) => {
							if let Some(Token {
								value: TokenType::Generic(_var_value),
								..
							}) = token_iter.next()
							{
								todo!();
							}
						}
						// let init_node = Node::VarDec(Box::new(VarDec {
						// 	var_type: if init_type == Keyword::Let {
						// 		VarType::Let
						// 	} else {
						// 		VarType::Const
						// 	},
						// 	name: param_name,
						// 	type_annotation: None,
						// 	value: Box::new(Node::Literal(Box::new(Literal::Undefined))),
						// }));
						// expr_list.push(init_node);
						_ => unimplemented!(),
					}
				} else {
					unimplemented!();
				}
			}
			_ => unimplemented!(),
		}
	}

	expr_list
}
