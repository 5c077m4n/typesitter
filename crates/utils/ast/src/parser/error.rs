use lexer::token::token_variance::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error<'e> {
	pub message: String,
	pub fragment: &'e str,
	pub line: usize,
	pub column: usize,
}
impl<'e> Error<'e> {
	pub fn new(message: String, position: &Span<'e>) -> Self {
		Error {
			message,
			fragment: std::str::from_utf8(position.fragment()).unwrap(),
			line: position.location_line() as usize,
			column: position.get_utf8_column() as usize,
		}
	}
}
