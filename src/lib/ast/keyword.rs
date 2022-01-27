#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keyword {
	Const,
	Let,
	Function,
	Class,
	Try,
	Catch,
	Finally,
	If,
	Else,
	Switch,
	Case,
	Export,
	Import,
	Await,
	Async,
	Return,
}
impl Keyword {
	pub fn as_str(&self) -> &str {
		match *self {
			Self::Const => "const",
			Self::Let => "let",
			Self::Function => "function",
			Self::Class => "class",
			Self::Try => "try",
			Self::Catch => "catch",
			Self::Finally => "finally",
			Self::If => "if",
			Self::Else => "else",
			Self::Switch => "switch",
			Self::Case => "case",
			Self::Export => "export",
			Self::Import => "import",
			Self::Await => "await",
			Self::Async => "async",
			Self::Return => "return",
		}
	}
}
