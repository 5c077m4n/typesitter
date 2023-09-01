use ast::parser::parse::parse_into_block;
use lexer::scanner::scan;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn eval(text: &str) -> String {
	let mut token_iter = scan(text, Some("WASM".to_string()));
	let ast = parse_into_block(&mut token_iter);
	let ast = ast.as_ref().unwrap();
	serde_json::to_string(&ast).unwrap()
}
