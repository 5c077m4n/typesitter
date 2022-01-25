use strum::EnumIter;

#[derive(Clone, PartialEq, Eq, EnumIter)]
pub enum Punctuation {
	/// `(`
	BracetOpen,
	/// `)`
	BracetClose,
	/// `[`
	BracetSquareOpen,
	/// `]`
	BracetSquareClose,
	/// `{`
	BracetCurlyOpen,
	/// `}`
	BracetCurlyClose,
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
}
impl Punctuation {
	pub fn as_str(&self) -> &str {
		match *self {
			Self::BracetOpen => "(",
			Self::BracetClose => ")",
			Self::BracetSquareOpen => "[",
			Self::BracetSquareClose => "]",
			Self::BracetCurlyOpen => "{",
			Self::BracetCurlyClose => "}",
			Self::Semicolon => ";",
			Self::Colon => ":",
			Self::Equal => "=",
			Self::ExclamationMark => "!",
			Self::LessThan => "<",
			Self::GreaterThan => ">",
			Self::Pipe => "|",
			Self::Ampersand => "&",
		}
	}
}
