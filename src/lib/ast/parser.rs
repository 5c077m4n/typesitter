use {
	super::{
		super::lexer::token::{
			keyword::Keyword,
			literal::Literal as TokenLiteral,
			punctuation::Punctuation,
			Token,
			TokenType,
		},
		types::{
			literal::Literal,
			node::Node,
			var_dec::{VarDec, VarType},
		},
	},
	anyhow::Result,
	log::error,
};

pub fn parse<'a>(
	mut token_iter: impl Iterator<Item = Token<'a>>,
	expr_list: Option<Vec<Node<'a>>>,
) -> Result<Box<Node<'a>>> {
	let mut expr_list = expr_list.unwrap_or_else(Vec::new);

	while let Some(Token { value, .. }) = token_iter.next() {
		match value {
			TokenType::Keyword(init_type @ (Keyword::Const | Keyword::Let)) => {
				if let Some(Token {
					value: TokenType::Identifier(param_name),
					..
				}) = token_iter.next()
				{
					match token_iter.next() {
						Some(Token {
							value: TokenType::Punctuation(Punctuation::Colon),
							..
						}) => {
							if let Some(Token {
								value: TokenType::Identifier(var_type),
								..
							}) = token_iter.next()
							{
								if let Some(Token {
									value: TokenType::Punctuation(Punctuation::Equal),
									..
								}) = token_iter.next()
								{
									match token_iter.next() {
										Some(Token {
											value: TokenType::Literal(TokenLiteral::Number(n)),
											..
										}) => {
											if let Some(Token {
												value:
													TokenType::Punctuation(Punctuation::Semicolon),
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
										Some(Token {
											value: TokenType::Literal(TokenLiteral::String(s)),
											..
										}) => {
											if let Some(Token {
												value:
													TokenType::Punctuation(Punctuation::Semicolon),
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
														Literal::String(s),
													))),
												}));

												expr_list.push(init_node);
											}
										}
										other => {
											error!("{:?}", &other);
											unimplemented!();
										}
									}
								}
							}
						}
						Some(Token {
							value: TokenType::Punctuation(Punctuation::Equal),
							..
						}) => match token_iter.next() {
							Some(Token {
								value: TokenType::Literal(TokenLiteral::Number(n)),
								..
							}) => {
								if let Some(Token {
									value: TokenType::Punctuation(Punctuation::Semicolon),
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
										type_annotation: None,
										value: Box::new(Node::Literal(Box::new(Literal::Number(
											n,
										)))),
									}));

									expr_list.push(init_node);
								}
							}
							Some(Token {
								value: TokenType::Literal(TokenLiteral::String(s)),
								..
							}) => {
								if let Some(Token {
									value: TokenType::Punctuation(Punctuation::Semicolon),
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
										type_annotation: None,
										value: Box::new(Node::Literal(Box::new(Literal::String(
											s,
										)))),
									}));

									expr_list.push(init_node);
								}
							}
							other => {
								error!("{:?}", &other);
								unimplemented!();
							}
						},
						other => {
							error!("{:?}", other);
							unimplemented!();
						}
					}
				} else {
					unimplemented!();
				}
			}
			other => {
				error!("{:?}", other);
				unimplemented!();
			}
		}
	}

	Ok(Box::new(Node::Block(expr_list)))
}
