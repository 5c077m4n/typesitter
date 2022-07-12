use anyhow::Result;
use assert_cmd::Command;

const BIN_NAME: &str = env!("CARGO_PKG_NAME");

#[test]
#[ignore]
fn eval_number_var_init_test() -> Result<()> {
	let mut cmd = Command::cargo_bin(BIN_NAME)?;

	cmd.args(&["--eval", "const n: number = 123;"])
		.assert()
		.success();
	Ok(())
}

#[test]
#[ignore]
fn eval_string_var_init_test() -> Result<()> {
	let mut cmd = Command::cargo_bin(BIN_NAME)?;

	cmd.args(&["--eval", r#"const s: string = 'qwerty';"#])
		.assert()
		.success();
	Ok(())
}

#[test]
#[ignore]
fn stdin_test() -> Result<()> {
	let mut cmd = Command::cargo_bin(BIN_NAME)?;

	cmd.write_stdin("const n: number = 1234;")
		.write_stdin("\n")
		.assert()
		.success();
	Ok(())
}

#[test]
#[ignore]
fn file_read_test() -> Result<()> {
	let mut cmd = Command::cargo_bin(BIN_NAME)?;

	cmd.arg("tests/assets/1.ts").assert().success();
	Ok(())
}
