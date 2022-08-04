use super::{
	super::types::{
		fn_dec::{FnDecl, FnType},
		literal::Literal,
		node::Node,
		type_annotation::TypeAnnotation,
		var_decl::VarDecl,
	},
	ident::ident_parse,
	param_list::parse_fn_decl_input_list,
};
use anyhow::{bail, Result};
use lexer::token::{
	keyword::Keyword,
	literal::Literal as TokenLiteral,
	punctuation::Punctuation,
	token_variance::{Token, TokenType},
};
use log::error;
use std::iter::Peekable;

pub fn parse<'p>(
	token_iter: &mut Box<Peekable<impl Iterator<Item = Token<'p>>>>,
) -> Result<Vec<Node<'p>>> {
	let mut expr_list: Vec<Node<'p>> = Vec::new();

	while let Some(Token { value, position }) = token_iter.next() {
		match value {
			TokenType::Punctuation(Punctuation::Semicolon) => (), // This makes the `;` optional - but only at the right placement
			TokenType::Punctuation(Punctuation::BracketCurlyClose) => break,
			TokenType::Keyword(Keyword::Function) => match token_iter.next() {
				Some(Token {
					value: TokenType::Identifier(fn_name),
					..
				}) => match token_iter.next() {
					Some(Token {
						value: TokenType::Punctuation(Punctuation::BracketOpen),
						position,
					}) => {
						let input_params = parse_fn_decl_input_list(token_iter)?;
						match token_iter.next() {
							Some(Token {
								value: TokenType::Punctuation(Punctuation::BracketCurlyOpen),
								..
							}) => {
								let body = parse(token_iter)?;
								let named_fn_node = Node::FnDecl(FnDecl {
									fn_type: FnType::Classic,
									name: Some(vec![fn_name]),
									input_params,
									return_type: None,
									body: Box::new(Node::Block(body)),
								});
								expr_list.push(named_fn_node);
							}
							Some(Token {
								value: TokenType::Punctuation(Punctuation::Colon),
								position,
							}) => match token_iter.next() {
								Some(Token {
									value: TokenType::Identifier(fn_return_type),
									..
								}) => match token_iter.next() {
									Some(Token {
										value: TokenType::Punctuation(Punctuation::BracketCurlyOpen),
										..
									}) => {
										let body = parse(token_iter)?;
										let named_fn_node = Node::FnDecl(FnDecl {
											fn_type: FnType::Classic,
											name: Some(vec![fn_name]),
											input_params,
											return_type: Some(TypeAnnotation::try_from(
												fn_return_type,
											)?),
											body: Box::new(Node::Block(body)),
										});
										expr_list.push(named_fn_node);
									}
									other => {
										bail!("Wasn't expecting {:?} @ {:?}", &other, &position)
									}
								},
								other => bail!("Wasn't expecting {:?} @ {:?}", &other, &position),
							},
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
					let input_params = parse_fn_decl_input_list(token_iter)?;
					if let Some(Token {
						value: TokenType::Punctuation(Punctuation::BracketCurlyOpen),
						..
					}) = token_iter.next()
					{
						let body = parse(token_iter)?;
						let unnamed_fn_node = Node::FnDecl(FnDecl {
							fn_type: FnType::Classic,
							name: None,
							input_params,
							return_type: None,
							body: Box::new(Node::Block(body)),
						});
						expr_list.push(unnamed_fn_node);
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
											let init_node = Node::VarDecl(VarDecl {
												var_type: init_type.try_into()?,
												name: vec![param_name],
												type_annotation: Some(TypeAnnotation::try_from(
													var_type,
												)?),
												value: Box::new(Node::Literal(Literal::Number(n))),
											});
											expr_list.push(init_node);
										}
										Some(Token {
											value: TokenType::Literal(TokenLiteral::String(s)),
											..
										}) => {
											let init_node = Node::VarDecl(VarDecl {
												var_type: init_type.try_into()?,
												name: vec![param_name],
												type_annotation: Some(TypeAnnotation::try_from(
													var_type,
												)?),
												value: Box::new(Node::Literal(Literal::String(s))),
											});
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
								let init_node = Node::VarDecl(VarDecl {
									var_type: init_type.try_into()?,
									name: vec![param_name],
									type_annotation: None,
									value: Box::new(Node::Literal(Literal::Number(n))),
								});
								expr_list.push(init_node);
							}
							Some(Token {
								value: TokenType::Literal(TokenLiteral::String(s)),
								..
							}) => {
								let init_node = Node::VarDecl(VarDecl {
									var_type: init_type.try_into()?,
									name: vec![param_name],
									type_annotation: None,
									value: Box::new(Node::Literal(Literal::String(s))),
								});
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
							let init_node = Node::VarDecl(VarDecl {
								var_type: init_type.try_into()?,
								name: vec![param_name],
								type_annotation: None,
								value: Box::new(Node::Literal(Literal::Undefined)),
							});
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
			TokenType::Identifier(ident) => {
				let mut ident_list = ident_parse(ident, token_iter)?;
				expr_list.append(&mut ident_list);
			}
			TokenType::Keyword(Keyword::Return) => match token_iter.next() {
				Some(Token {
					value: TokenType::Identifier(ret_ident),
					..
				}) => {
					expr_list.push(Node::Return(Box::new(Node::VarCall(ret_ident))));
				}
				Some(Token {
					value: TokenType::Literal(lit),
					..
				}) => {
					expr_list.push(Node::Return(Box::new(Node::Literal(lit.try_into()?))));
				}
				_ => {}
			},
			other => {
				error!("{:?} @ {:?}", &other, &position);
				unimplemented!("[Main] {:?} @ {:?}", &other, &position);
			}
		}
	}

	Ok(expr_list)
}

pub fn parse_into_block<'p>(
	token_iter: &mut Box<Peekable<impl Iterator<Item = Token<'p>>>>,
) -> Result<Node<'p>> {
	let ast = parse(token_iter)?;
	Ok(Node::Block(ast))
}
