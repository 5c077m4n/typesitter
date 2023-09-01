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
			fn_dec::{FnDec, FnType},
			literal::Literal,
			node::Node,
			var_dec::VarDecl,
		},
	},
	param_list::parse_param_list,
};
use anyhow::{bail, Result};
use log::error;

pub fn parse<'a>(token_iter: &mut impl Iterator<Item = Token<'a>>) -> Result<Vec<Box<Node<'a>>>> {
	let mut expr_list: Vec<Box<Node<'a>>> = Vec::new();

	while let Some(Token { value, position }) = token_iter.next() {
		match value {
			// This makes the `;` optional - but only at the right placement
			TokenType::Punctuation(Punctuation::Semicolon) => (),
			TokenType::Punctuation(Punctuation::BracketCurlyClose) => (),
			TokenType::Keyword(Keyword::Function) => match token_iter.next() {
				Some(Token {
					value: TokenType::Identifier(fn_name),
					..
				}) => match token_iter.next() {
					Some(Token {
						value: TokenType::Punctuation(Punctuation::BracketOpen),
						position,
					}) => {
						let input_params = parse_param_list(token_iter)?;
						match token_iter.next() {
							Some(Token {
								value: TokenType::Punctuation(Punctuation::BracketCurlyOpen),
								..
							}) => {
								let body = parse(token_iter)?;
								let named_fn_node = Box::new(Node::FnDec(FnDec {
									fn_type: FnType::Classic,
									name: Some(fn_name),
									input_params,
									return_type: None,
									body: Box::new(Node::Block(body)),
								}));
								expr_list.push(named_fn_node);
							}
							Some(Token {
								value: TokenType::Punctuation(Punctuation::Colon),
								..
							}) => {
								todo!("function return type")
							}
							other => {
								error!("Was not expecting {:?} @ {:?}", &other, &position);
							}
						}
					}
					other => {
						bail!("The character `(` was expected here, but got: {:?}", &other);
					}
				},
				Some(Token {
					value: TokenType::Punctuation(Punctuation::BracketOpen),
					..
				}) => {
					let _params = parse_param_list(token_iter)?;
					if let Some(Token {
						value: TokenType::Punctuation(Punctuation::BracketCurlyOpen),
						..
					}) = token_iter.next()
					{
						unimplemented!("Function call on nested object");
					}
				}
				_ => {}
			},
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
											let init_node = Box::new(Node::VarDecl(VarDecl {
												var_type: init_type.try_into()?,
												name: param_name,
												type_annotation: Some(var_type),
												value: Box::new(Node::Literal(Literal::Number(n))),
											}));
											expr_list.push(init_node);
										}
										Some(Token {
											value: TokenType::Literal(TokenLiteral::String(s)),
											..
										}) => {
											let init_node = Box::new(Node::VarDecl(VarDecl {
												var_type: init_type.try_into()?,
												name: param_name,
												type_annotation: Some(var_type),
												value: Box::new(Node::Literal(Literal::String(s))),
											}));
											expr_list.push(init_node);
										}
										other => {
											error!("{:?}", &other);
											unimplemented!("Node::Literal");
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
								let init_node = Box::new(Node::VarDecl(VarDecl {
									var_type: init_type.try_into()?,
									name: param_name,
									type_annotation: None,
									value: Box::new(Node::Literal(Literal::Number(n))),
								}));
								expr_list.push(init_node);
							}
							Some(Token {
								value: TokenType::Literal(TokenLiteral::String(s)),
								..
							}) => {
								let init_node = Box::new(Node::VarDecl(VarDecl {
									var_type: init_type.try_into()?,
									name: param_name,
									type_annotation: None,
									value: Box::new(Node::Literal(Literal::String(s))),
								}));
								expr_list.push(init_node);
							}
							other => {
								error!("{:?}", &other);
								unimplemented!("VarDecl");
							}
						},
						Some(Token {
							value: TokenType::Punctuation(Punctuation::Semicolon),
							..
						}) => {
							let init_node = Box::new(Node::VarDecl(VarDecl {
								var_type: init_type.try_into()?,
								name: param_name,
								type_annotation: None,
								value: Box::new(Node::Literal(Literal::Undefined)),
							}));
							expr_list.push(init_node);
						}
						other => {
							error!("{:?}", &other);
							unimplemented!("Not initialized VarDecl");
						}
					},
					other => {
						error!("{:?}", &other);
						unimplemented!("VarDecl");
					}
				}
			}
			TokenType::Identifier(ident) => match token_iter.next() {
				Some(Token {
					value: TokenType::Punctuation(Punctuation::BracketOpen),
					..
				}) => {
					let params = parse_param_list(token_iter)?;
					let fn_call_node = Box::new(Node::FnCall(FnCall {
						fn_name: ident,
						params,
					}));
					expr_list.push(fn_call_node);
				}
				Some(Token {
					value: TokenType::Punctuation(Punctuation::Dot),
					..
				}) => {
					todo!("Object fields lookup");
				}
				Some(Token {
					value: TokenType::Punctuation(Punctuation::Equal),
					..
				}) => {
					todo!();
				}
				other => {
					error!("{:?}", &other);
					unimplemented!("Identifier");
				}
			},
			other => {
				error!("{:?} @ {:?}", &other, &position);
				unimplemented!("Main @ {:?}", &position);
			}
		}
	}

	Ok(expr_list)
}
