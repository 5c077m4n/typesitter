use strum::EnumIter;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter)]
#[strum(serialize_all = "lowercase")]
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
