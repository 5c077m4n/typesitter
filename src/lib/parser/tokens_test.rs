use super::tokens::*;

#[test]
fn one_line_comment_test() {
	assert_eq!(one_line_comment("// comment"), Ok((" comment", ())));
}

#[test]
fn multiline_comment_test() {
	assert_eq!(multiline_comment("/* comment */"), Ok((" comment ", ())));
}
