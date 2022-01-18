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

#[test]
fn string_single_quote_test() {
	assert_eq!(string(r#"'234'"#), Ok(("", "234")));
}

#[test]
fn string_double_quote_test() {
	assert_eq!(string(r#""2 34""#), Ok(("", "2 34")));
}

#[test]
fn const_var_init_test() {
	assert_eq!(var_init(r#"const a = 1234;"#), Ok(("", ("const", "a", "1234"))));
}

#[test]
fn let_var_init_test() {
	assert_eq!(var_init(r#"let a = 'qwerty';"#), Ok(("", ("let", "a", "'qwerty'"))));
}
