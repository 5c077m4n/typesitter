#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
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
	/// `'`
	QuoteSingle,
	/// `"`
	QuoteDouble,
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
	/// `.`
	Dot,
	/// `,`
	Comma,
	/// \s
	Space,
	/// \t
	Tab,
	/// End of line
	EOL,
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
			Self::QuoteDouble => "\"",
			Self::QuoteSingle => "'",
			Self::Semicolon => ";",
			Self::Colon => ":",
			Self::Equal => "=",
			Self::ExclamationMark => "!",
			Self::LessThan => "<",
			Self::GreaterThan => ">",
			Self::Pipe => "|",
			Self::Ampersand => "&",
			Self::Dot => ".",
			Self::Comma => ",",
			Self::Space => " ",
			Self::Tab => "	",
			Self::EOL => r"\n",
		}
	}
}
