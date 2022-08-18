use ast::parser::parse::Parser;
use bytecode::codegen::CodeGen;
use lexer::scanner::scan;
use vm::vm::VM;
use wasm_bindgen::prelude::*;

#[cfg(feature = "demo")]
pub mod demo {
	use super::*;

	#[wasm_bindgen]
	pub fn tokenize(text: String) -> String {
		let text = text.as_bytes();
		let tokens: Vec<_> = scan(text, Some("WASM".to_string())).collect();
		serde_json::to_string(&tokens).unwrap()
	}

	#[wasm_bindgen]
	pub fn build_ast(text: String) -> String {
		let text = text.as_bytes();
		let token_iter = scan(text, Some("WASM".to_string()));

		let mut parser = Parser::new(token_iter);
		let ast = parser.parse_into_block().unwrap();

		serde_json::to_string(&ast).unwrap()
	}

	#[wasm_bindgen]
	pub fn check(text: String) -> String {
		let text = text.as_bytes();
		let token_iter = scan(text, Some("WASM".to_string()));

		let mut parser = Parser::new(token_iter);
		let _ast = parser.parse_into_block().unwrap();

		let errors = parser.get_errors();
		serde_json::to_string(&errors).unwrap()
	}
}

#[wasm_bindgen]
pub fn ts_eval(text: String) -> String {
	let text = text.as_bytes();
	let token_iter = scan(text, Some("WASM".to_string()));

	let mut parser = Parser::new(token_iter);
	let ast = parser.parse_into_block().unwrap();

	let errors = parser.get_errors();
	let errors = serde_json::to_string(errors).unwrap();
	let errors: JsValue = errors.into();

	web_sys::console::error(&errors.into());

	let mut codegen = CodeGen::default();
	let program = codegen.run(&ast).unwrap();

	let mut vm = VM::default();
	let result = vm.interpret(&program).unwrap();
	format!("{}", &result)
}
