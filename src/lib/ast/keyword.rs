#![allow(dead_code)]

use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keyword {
	CONST,
	LET,
	FUNCTION,
	CLASS,
	TRUE,
	FALSE,
	TRY,
	CATCH,
	FINALLY,
	IF,
	ELSE,
	EXPORT,
	IMPORT,
	AWAIT,
	ASYNC,
}
impl Keyword {
	pub fn as_str(&self) -> &str {
		match self {
			Self::CONST => "const",
			Self::LET => "let",
			Self::FUNCTION => "function",
			Self::CLASS => "class",
			Self::TRUE => "true",
			Self::FALSE => "false",
			Self::TRY => "try",
			Self::CATCH => "catch",
			Self::FINALLY => "finally",
			Self::IF => "if",
			Self::ELSE => "else",
			Self::EXPORT => "export",
			Self::IMPORT => "import",
			Self::AWAIT => "await",
			Self::ASYNC => "async",
		}
	}
}
pub struct KeywordErr(String);
impl FromStr for Keyword {
	type Err = KeywordErr;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"const" => Ok(Self::CONST),
			"let" => Ok(Self::LET),
			"function" => Ok(Self::FUNCTION),
			"class" => Ok(Self::CLASS),
			"true" => Ok(Self::TRUE),
			"false" => Ok(Self::FALSE),
			"try" => Ok(Self::TRY),
			"catch" => Ok(Self::CATCH),
			"finally" => Ok(Self::FINALLY),
			"if" => Ok(Self::IF),
			"else" => Ok(Self::ELSE),
			"export" => Ok(Self::EXPORT),
			"import" => Ok(Self::IMPORT),
			"await" => Ok(Self::AWAIT),
			"async" => Ok(Self::ASYNC),
			other => Err(KeywordErr(other.to_owned())),
		}
	}
}
