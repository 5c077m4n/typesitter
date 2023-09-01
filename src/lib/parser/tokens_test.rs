use super::tokens::*;

#[test]
fn decimal_test() {
	assert_eq!(decimal("42"), Ok(("", "42")));
}

#[test]
fn binary_with_dashes_test() {
	assert_eq!(binary("0b01_01"), Ok(("", "01_01")));
}

#[test]
fn binary_test() {
	assert_eq!(binary("0b0101"), Ok(("", "0101")));
}
