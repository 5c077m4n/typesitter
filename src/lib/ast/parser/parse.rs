use super::{
	super::{
		super::lexer::token::{
			keyword::Keyword,
			literal::Literal as TokenLiteral,
			punctuation::Punctuation,
			token_variance::{Token, TokenType},
		},
		types::{
			fn_call::FnCall,
			literal::Literal,
			node::Node,
			var_dec::{VarDec, VarType},
		},
	},
	param_list::parse_param_list,
};
use anyhow::Result;
use log::error;

pub fn parse<'a>(
	mut token_iter: impl Iterator<Item = Token<'a>>,
	expr_list: Option<Vec<Node<'a>>>,
) -> Result<Box<Node<'a>>> {
	let mut expr_list = expr_list.unwrap_or_else(Vec::new);

	while let Some(Token { value, .. }) = token_iter.next() {
		match value {
			// This makes the `;` optional - but only at the right placement
			TokenType::Punctuation(Punctuation::Semicolon) => {}
			TokenType::Keyword(init_type @ (Keyword::Const | Keyword::Let)) => {
				match token_iter.next() {
					Some(Token {
						value: TokenType::Identifier(param_name),
						..
					}) => match token_iter.next() {
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
										Some(Token {
											value: TokenType::Literal(TokenLiteral::String(s)),
											..
										}) => {
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
								let init_node = Node::VarDec(Box::new(VarDec {
									var_type: if init_type == Keyword::Let {
										VarType::Let
									} else {
										VarType::Const
									},
									name: param_name,
									type_annotation: None,
									value: Box::new(Node::Literal(Box::new(Literal::Number(n)))),
								}));
								expr_list.push(init_node);
							}
							Some(Token {
								value: TokenType::Literal(TokenLiteral::String(s)),
								..
							}) => {
								let init_node = Node::VarDec(Box::new(VarDec {
									var_type: if init_type == Keyword::Let {
										VarType::Let
									} else {
										VarType::Const
									},
									name: param_name,
									type_annotation: None,
									value: Box::new(Node::Literal(Box::new(Literal::String(s)))),
								}));
								expr_list.push(init_node);
							}
							other => {
								error!("{:?}", &other);
								unimplemented!();
							}
						},
						Some(Token {
							value: TokenType::Punctuation(Punctuation::Semicolon),
							..
						}) => {
							let init_node = Node::VarDec(Box::new(VarDec {
								var_type: if init_type == Keyword::Let {
									VarType::Let
								} else {
									VarType::Const
								},
								name: param_name,
								type_annotation: None,
								value: Box::new(Node::Literal(Box::new(Literal::Undefined))),
							}));
							expr_list.push(init_node);
						}
						other => {
							error!("{:?}", &other);
							unimplemented!();
						}
					},
					other => {
						error!("{:?}", &other);
						unimplemented!();
					}
				}
			}
			TokenType::Identifier(ident) => match token_iter.next() {
				Some(Token {
					value: TokenType::Punctuation(Punctuation::BracketOpen),
					..
				}) => {
					let params = parse_param_list(token_iter.by_ref())?;
					let fn_call_node = Node::FnCall(Box::new(FnCall {
						fn_name: ident,
						params,
					}));
					expr_list.push(fn_call_node);
				}
				Some(Token {
					value: TokenType::Punctuation(Punctuation::Dot),
					..
				}) => {
					todo!();
				}
				Some(Token {
					value: TokenType::Punctuation(Punctuation::Equal),
					..
				}) => {
					todo!();
				}
				other => {
					error!("{:?}", &other);
					unimplemented!();
				}
			},
			other => {
				error!("{:?}", &other);
				unimplemented!();
			}
		}
	}

	Ok(Box::new(Node::Block(expr_list)))
}
