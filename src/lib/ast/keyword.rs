use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keyword {
	Const,
	Let,
	Function,
	Class,
	True,
	False,
	Try,
	Catch,
	Finally,
	If,
	Else,
	Export,
	Import,
	Await,
	Async,
}
impl Keyword {
	pub fn as_str(&self) -> &str {
		match *self {
			Self::Const => "const",
			Self::Let => "let",
			Self::Function => "function",
			Self::Class => "class",
			Self::True => "true",
			Self::False => "false",
			Self::Try => "try",
			Self::Catch => "catch",
			Self::Finally => "finally",
			Self::If => "if",
			Self::Else => "else",
			Self::Export => "export",
			Self::Import => "import",
			Self::Await => "await",
			Self::Async => "async",
		}
	}
}
pub struct KeywordErr(String);
impl FromStr for Keyword {
	type Err = KeywordErr;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"const" => Ok(Self::Const),
			"let" => Ok(Self::Let),
			"function" => Ok(Self::Function),
			"class" => Ok(Self::Class),
			"true" => Ok(Self::True),
			"false" => Ok(Self::False),
			"try" => Ok(Self::Try),
			"catch" => Ok(Self::Catch),
			"finally" => Ok(Self::Finally),
			"if" => Ok(Self::If),
			"else" => Ok(Self::Else),
			"export" => Ok(Self::Export),
			"import" => Ok(Self::Import),
			"await" => Ok(Self::Await),
			"async" => Ok(Self::Async),
			other => Err(KeywordErr(other.to_owned())),
		}
	}
}
