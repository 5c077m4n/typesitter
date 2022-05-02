use ast::parser::parse::parse_into_block;
use lexer::{scanner::scan, token::token_variance::Token};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tokenize(text: &str) -> String {
	let tokens: Vec<Token> = scan(text, Some("WASM".to_string())).collect();
	serde_json::to_string(&tokens).unwrap()
}

#[wasm_bindgen]
pub fn build_ast(text: &str) -> String {
	let mut token_iter = scan(text, Some("WASM".to_string()));
	let ast = parse_into_block(&mut token_iter);
	let ast = ast.as_ref().unwrap();
	serde_json::to_string(&ast).unwrap()
}

#[wasm_bindgen]
pub fn ts_eval(text: &str) -> String {
	let mut token_iter = scan(text, Some("WASM".to_string()));
	let ast = parse_into_block(&mut token_iter);
	let ast = ast.as_ref().unwrap();
	serde_json::to_string(&ast).unwrap()
}
