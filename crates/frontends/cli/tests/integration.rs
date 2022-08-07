use anyhow::Result;
use assert_cmd::Command;
use std::time::Duration;

const BIN_NAME: &str = env!("CARGO_PKG_NAME");

#[test]
fn eval_number_var_init() -> Result<()> {
	let mut cmd = Command::cargo_bin(BIN_NAME)?;

	cmd.args(&["--eval", "const n: number = 123;"])
		.assert()
		.success();
	Ok(())
}

#[test]
fn stdin() -> Result<()> {
	let mut cmd = Command::cargo_bin(BIN_NAME)?;

	cmd.write_stdin("const n: number = 1234;")
		.write_stdin("\n")
		.timeout(Duration::from_secs(30))
		.assert()
		.success();
	Ok(())
}

#[test]
fn file_read_decl_call_var() -> Result<()> {
	let mut cmd = Command::cargo_bin(BIN_NAME)?;

	cmd.arg("tests/assets/0.ts").assert().success();
	Ok(())
}

#[test]
fn console_log_panik() {
	let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();

	cmd.args(&[
		"--eval",
		"const n: number = 1234; console.log(noSuchParam);",
	])
	.assert()
	.failure();
}

#[test]
fn console_log() -> Result<()> {
	let mut cmd = Command::cargo_bin(BIN_NAME)?;

	cmd.args(&["--eval", "const n: number = 1234; console.log(n);"])
		.assert()
		.code(0)
		.stdout("1234\n")
		.success();
	Ok(())
}

#[test]
fn console_log_2() -> Result<()> {
	let mut cmd = Command::cargo_bin(BIN_NAME)?;

	cmd.args(&["--eval", "const n1 = 1; const n2 = 3; console.log(n1, n2);"])
		.assert()
		.code(0)
		.stdout("1, 3\n")
		.success();
	Ok(())
}

#[test]
fn console_error() -> Result<()> {
	let mut cmd = Command::cargo_bin(BIN_NAME)?;

	cmd.args(&[
		"--eval",
		"const n1 = 1; const n2 = 3; console.error(n1, n2);",
	])
	.assert()
	.code(0)
	.stderr("1, 3\n")
	.success();
	Ok(())
}
