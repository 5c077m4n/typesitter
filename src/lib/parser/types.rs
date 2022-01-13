use std::str::{self, FromStr};

use nom::{error::Error, number::complete::double};

#[derive(Debug)]
pub struct Number(pub f64);
impl PartialEq for Number {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}
impl FromStr for Number {
	type Err = Error<String>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match double(s) {
			Ok((_tail, n)) => Ok(Self(n)),
			Err(nom::Err::Error(Error { input, code })) => Err(Error {
				input: input.to_string(),
				code,
			}),
			_ => unreachable!(),
		}
	}
}
