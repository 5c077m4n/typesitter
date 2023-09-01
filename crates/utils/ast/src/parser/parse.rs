use super::{
	super::types::{
		fn_dec::{FnDecl, FnType},
		literal::Literal,
		node::Node,
		type_annotation::TypeAnnotation,
		var_decl::VarDecl,
	},
	error::Error,
};
use crate::types::{
	bin_op::{BinOp, Operator},
	fn_call::FnCall,
	var_decl::VarType,
};
use anyhow::Result;
use lexer::token::{
	keyword::Keyword,
	literal::Literal as TokenLiteral,
	punctuation::Punctuation,
	token_variance::{Token, TokenType},
};
use std::iter::Peekable;

pub struct Parser<'p, I: Iterator<Item = Token<'p>>> {
	token_iter: Box<Peekable<I>>,
	errors: Vec<Error<'p>>,
}
impl<'p, I: Iterator<Item = Token<'p>>> Parser<'p, I> {
	pub fn new(token_iter: Box<Peekable<I>>) -> Self {
		Self {
			token_iter,
			errors: vec![],
		}
	}

	pub fn get_errors(&self) -> &[Error] {
		&self.errors
	}

	pub fn parse_into_block(&mut self) -> Result<Node<'p>> {
		let ast = self.parse()?;
		Ok(Node::Block(ast.to_vec()))
	}

	pub fn parse(&mut self) -> Result<Vec<Node<'p>>> {
		let mut expr_list: Vec<Node<'p>> = vec![];

		while let Some(Token { value, position }) = self.token_iter.next() {
			match value {
				TokenType::Punctuation(Punctuation::Semicolon) => (), // This makes the `;` optional - but only at the right placement
				TokenType::Punctuation(Punctuation::BracketCurlyClose) => break,
				TokenType::Keyword(Keyword::Function) => match self.token_iter.next() {
					Some(Token {
						value: TokenType::Identifier(fn_name),
						position,
					}) => match self.token_iter.next() {
						Some(Token {
							value: TokenType::Punctuation(Punctuation::BracketOpen),
							position,
						}) => {
							let input_params = self.parse_fn_decl_input_list()?;
							match self.token_iter.next() {
								Some(Token {
									value: TokenType::Punctuation(Punctuation::BracketCurlyOpen),
									..
								}) => {
									let body = self.parse()?;
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
								}) => match self.token_iter.next() {
									Some(Token {
										value: TokenType::Identifier(fn_return_type),
										..
									}) => match self.token_iter.next() {
										Some(Token {
											value:
												TokenType::Punctuation(Punctuation::BracketCurlyOpen),
											..
										}) => {
											let body = self.parse()?;
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
											self.errors.push(Error::new(
												format!("Wasn't expecting {:?}", &other),
												&position,
											));
										}
									},
									other => {
										self.errors.push(Error::new(
											format!("Wasn't expecting {:?}", &other),
											&position,
										));
									}
								},
								other => {
									self.errors.push(Error::new(
										format!("Wasn't expecting {:?}", &other),
										&position,
									));
								}
							}
						}
						other => {
							self.errors.push(Error::new(
								format!(
									"The character `(` was expected here, but got: {:?}",
									&other
								),
								&position,
							));
						}
					},
					Some(Token {
						value: TokenType::Punctuation(Punctuation::BracketOpen),
						..
					}) => {
						let input_params = self.parse_fn_decl_input_list()?;
						if let Some(Token {
							value: TokenType::Punctuation(Punctuation::BracketCurlyOpen),
							..
						}) = self.token_iter.next()
						{
							let body = self.parse()?;
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
				TokenType::Keyword(init_type @ (Keyword::Const | Keyword::Let)) => match self
					.token_iter
					.next()
				{
					Some(Token {
						value: TokenType::Identifier(param_name),
						..
					}) => match self.token_iter.next() {
						Some(Token {
							value: TokenType::Punctuation(Punctuation::Colon),
							..
						}) => {
							if let Some(Token {
								value: TokenType::Identifier(var_type),
								..
							}) = self.token_iter.next()
							{
								if let Some(Token {
									value: TokenType::Punctuation(Punctuation::Equal),
									position,
								}) = self.token_iter.next()
								{
									match self.token_iter.next() {
										Some(Token {
											value: TokenType::Literal(TokenLiteral::Number(n)),
											position,
										}) => {
											let param_name = std::str::from_utf8(param_name)?;

											let type_annotation = TypeAnnotation::from(var_type);
											if type_annotation != TypeAnnotation::Number {
												self.errors
													.push(Error::new(format!("A parameter of value `{:?}` was expected here", &type_annotation), &position));
											}

											let init_node = Node::VarDecl(VarDecl {
												var_type: init_type.try_into()?,
												name: vec![param_name],
												type_annotation: Some(type_annotation),
												value: Box::new(Node::Literal(Literal::Number(n))),
											});
											expr_list.push(init_node);
										}
										Some(Token {
											value: TokenType::Literal(TokenLiteral::String(s)),
											position,
										}) => {
											let param_name = std::str::from_utf8(param_name)?;

											let type_annotation = TypeAnnotation::from(var_type);
											if type_annotation != TypeAnnotation::String {
												self.errors
													.push(Error::new(format!("A parameter of value `{:?}` was expected here", &type_annotation), &position));
											}

											let init_node = Node::VarDecl(VarDecl {
												var_type: init_type.try_into()?,
												name: vec![param_name],
												type_annotation: Some(type_annotation),
												value: Box::new(Node::Literal(Literal::String(s))),
											});
											expr_list.push(init_node);
										}
										other => {
											self.errors.push(Error::new(
												format!("Unimplemented Node::Literal {:?}", &other),
												&position,
											));
										}
									}
								}
							}
						}
						Some(Token {
							value: TokenType::Punctuation(Punctuation::Equal),
							position,
						}) => match self.token_iter.next() {
							Some(Token {
								value: TokenType::Literal(TokenLiteral::Number(n)),
								..
							}) => {
								let param_name = std::str::from_utf8(param_name)?;
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
								let param_name = std::str::from_utf8(param_name)?;
								let init_node = Node::VarDecl(VarDecl {
									var_type: init_type.try_into()?,
									name: vec![param_name],
									type_annotation: None,
									value: Box::new(Node::Literal(Literal::String(s))),
								});
								expr_list.push(init_node);
							}
							other => {
								self.errors.push(Error::new(
									format!("Unimplemented VarDecl {:?}", &other),
									&position,
								));
							}
						},
						Some(Token {
							value: TokenType::Punctuation(Punctuation::Semicolon),
							..
						}) => {
							let param_name = std::str::from_utf8(param_name)?;
							let init_node = Node::VarDecl(VarDecl {
								var_type: init_type.try_into()?,
								name: vec![param_name],
								type_annotation: None,
								value: Box::new(Node::Literal(Literal::Undefined)),
							});
							expr_list.push(init_node);
						}
						other => {
							self.errors.push(Error::new(
								format!("Unimplemented not initialized VarDecl {:?}", &other),
								&position,
							));
						}
					},
					other => {
						self.errors.push(Error::new(
							format!("Unimplemented VarDecl {:?}", &other),
							&position,
						));
					}
				},
				TokenType::Identifier(ident) => {
					let mut ident_list = self.ident_parse(ident)?;
					expr_list.append(&mut ident_list);
				}
				TokenType::Keyword(Keyword::Return) => match self.token_iter.next() {
					Some(Token {
						value: TokenType::Identifier(ret_ident),
						..
					}) => {
						let ret_ident = std::str::from_utf8(ret_ident)?;
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
					self.errors.push(Error::new(
						format!("Unimplemented Main {:?}", &other),
						&position,
					));
				}
			}
		}

		Ok(expr_list)
	}

	fn ident_parse(&mut self, ident: &'p [u8]) -> Result<Vec<Node<'p>>> {
		let mut expr_list: Vec<Node<'p>> = vec![];
		let mut ident_parts: Vec<&[u8]> = vec![ident];

		while let Some(Token { value, .. }) = self.token_iter.peek() {
			match value {
				TokenType::Identifier(ident_part) => ident_parts.push(ident_part),
				TokenType::Punctuation(Punctuation::Dot) => {}
				_ => break,
			}
			self.token_iter.next();
		}

		while let Some(Token { value, position }) = self.token_iter.next() {
			match value {
				TokenType::Punctuation(Punctuation::Semicolon) => break,
				TokenType::Punctuation(Punctuation::BracketOpen) => {
					let fn_name = ident_parts.to_owned();
					let params = self.parse_fn_call_input_list()?;

					let fn_call_node = Node::FnCall(FnCall { fn_name, params });
					expr_list.push(fn_call_node);
				}
				TokenType::Punctuation(Punctuation::Equal) => {
					let body = self.parse()?;
					let var_name: Vec<String> = ident_parts
						.iter()
						.map(|p| String::from_utf8(p.to_vec()).unwrap())
						.collect();
					let var_name = var_name.join(".");

					let _bin_op = BinOp {
						op: Operator::Eq,
						lhs: Box::new(Node::VarCall(&var_name)),
						rhs: Box::new(Node::Block(body)),
					};
					self.errors.push(Error::new(
						"Param assignment is unsupported yet".to_string(),
						&position,
					));
				}
				other => {
					self.errors.push(Error::new(
						format!("Unimplemented Identifier {:?}", &other),
						&position,
					));
				}
			}
		}

		Ok(expr_list)
	}

	fn parse_fn_decl_input_list(&mut self) -> Result<Vec<VarDecl<'p>>> {
		let mut input_token_index: usize = 0;
		let mut params: Vec<VarDecl> = Vec::new();

		while let Some(Token { value, position }) = self.token_iter.next() {
			match value {
				TokenType::Punctuation(Punctuation::BracketClose) => break,
				TokenType::Punctuation(Punctuation::Comma) => {
					if input_token_index == 0 {
						self.errors.push(Error::new(
							"Shouldn't put a comma as the first char in the fn input".to_string(),
							&position,
						));
					} else {
						input_token_index += 1;
					}
				}
				TokenType::Identifier(param_name) => {
					input_token_index += 1;

					let param_name = std::str::from_utf8(param_name)?;
					let param_dec = VarDecl {
						var_type: VarType::Let,
						name: vec![param_name],
						type_annotation: None,
						value: Box::new(Node::Literal(Literal::Undefined)),
					};
					params.push(param_dec);
				}
				TokenType::Punctuation(Punctuation::Colon) => {
					if input_token_index == 0 {
						self.errors.push(Error::new(
							"Shouldn't put a colon as the first char in the fn input".to_string(),
							&position,
						));
					}

					match self.token_iter.next() {
						Some(Token {
							value: TokenType::Identifier(fn_return_type),
							..
						}) => {
							params.last_mut().unwrap().type_annotation =
								Some(TypeAnnotation::try_from(fn_return_type)?);
						}
						other => {
							self.errors.push(Error::new(
								format!("Wasn't expecting {:?}", &other),
								&position,
							));
						}
					}
				}
				other => {
					self.errors.push(Error::new(
						format!("Wasn't expecting {:?}", &other),
						&position,
					));
				}
			}
		}

		Ok(params)
	}

	fn parse_fn_call_input_list(&mut self) -> Result<Vec<VarDecl<'p>>> {
		let mut input_token_index: usize = 0;
		let mut params: Vec<VarDecl> = Vec::new();

		for Token { value, position } in self.token_iter.by_ref() {
			match value {
				TokenType::Punctuation(Punctuation::BracketClose) => break,
				TokenType::Punctuation(Punctuation::Comma) => {
					if input_token_index == 0 {
						self.errors.push(Error::new(
							"Shouldn't put a comma as the first char in the fn input".to_string(),
							&position,
						));
					} else {
						input_token_index += 1;
					}
				}
				TokenType::Literal(lit) => {
					input_token_index += 1;

					match lit {
						TokenLiteral::Number(n) => {
							let param_dec = VarDecl {
								var_type: VarType::Let,
								name: vec![],
								type_annotation: Some(TypeAnnotation::Number),
								value: Box::new(Node::Literal(Literal::Number(n))),
							};
							params.push(param_dec);
						}
						other => {
							self.errors.push(Error::new(
								format!("Non-number types aren't supported yet ({:?})", &other,),
								&position,
							));
						}
					}
				}
				TokenType::Identifier(param_name) => {
					input_token_index += 1;

					let param_name = std::str::from_utf8(param_name)?;
					let param_dec = VarDecl {
						var_type: VarType::Let,
						name: vec![param_name],
						type_annotation: None,
						value: Box::new(Node::Literal(Literal::Undefined)),
					};
					params.push(param_dec);
				}
				other => {
					self.errors.push(Error::new(
						format!("Wasn't expecting {:?}", &other),
						&position,
					));
				}
			}
		}

		Ok(params)
	}
}
