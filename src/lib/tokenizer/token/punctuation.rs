#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Punctuation {
	/// `(`
	BracketOpen,
	/// `)`
	BracketClose,
	/// `[`
	BracketSquareOpen,
	/// `]`
	BracketSquareClose,
	/// `{`
	BracketCurlyOpen,
	/// `}`
	BracketCurlyClose,
	/// `;`
	Semicolon,
	/// `:`
	Colon,
	/// `=`
	Equal,
	/// `!`
	ExclamationMark,
	/// `<`
	LessThan,
	/// `>`
	GreaterThan,
	/// `|`
	Pipe,
	/// `&`
	Ampersand,
	/// .
	Dot,
}
impl Punctuation {
	pub fn as_str(&self) -> &str {
		match *self {
			Self::BracketOpen => "(",
			Self::BracketClose => ")",
			Self::BracketSquareOpen => "[",
			Self::BracketSquareClose => "]",
			Self::BracketCurlyOpen => "{",
			Self::BracketCurlyClose => "}",
			Self::Semicolon => ";",
			Self::Colon => ":",
			Self::Equal => "=",
			Self::ExclamationMark => "!",
			Self::LessThan => "<",
			Self::GreaterThan => ">",
			Self::Pipe => "|",
			Self::Ampersand => "&",
			Self::Dot => ".",
		}
	}
}
