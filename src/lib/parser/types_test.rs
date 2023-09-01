use super::types::*;

#[test]
fn number_test() {
	assert_eq!(".42".parse::<Number>(), Ok(Number(0.42)));
}
