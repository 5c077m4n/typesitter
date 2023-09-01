pub enum Punctuation {
	/// `(`
	BracetOpen,
	/// `)`
	BracetClose,
	/// `;`
	Semicolon,
	/// `:`
	Colon,
	/// `=`
	Equal,
	/// `!`
	ExclamationMark,
}
impl Punctuation {
	pub fn as_str(&self) -> &str {
		match *self {
			Self::BracetOpen => "(",
			Self::BracetClose => ")",
			Self::Semicolon => ";",
			Self::Colon => ":",
			Self::Equal => "=",
			Self::ExclamationMark => "!",
		}
	}
}
