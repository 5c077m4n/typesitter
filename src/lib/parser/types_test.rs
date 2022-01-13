use super::types::*;

#[test]
fn float_short_test() {
	assert_eq!(".42".parse::<Number>(), Ok(Number(0.42)));
}

#[test]
fn float_long_test() {
	assert_eq!("1.42".parse::<Number>(), Ok(Number(1.42)));
}

#[test]
fn float_as_int_test() {
	assert_eq!("42.".parse::<Number>(), Ok(Number(42.)));
}

#[test]
fn decimal_test() {
	assert_eq!("42".parse::<Number>(), Ok(Number(42.)));
}
