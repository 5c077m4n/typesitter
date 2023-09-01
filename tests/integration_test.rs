use anyhow::Result;
use assert_cmd::Command;

const BIN_NAME: &str = env!("CARGO_PKG_NAME");

#[test]
fn file_read_test() -> Result<()> {
	let mut cmd = Command::cargo_bin(BIN_NAME)?;

	cmd.arg("tests/assets/1.ts").assert().success();
	Ok(())
}

#[test]
fn eval_command_test() -> Result<()> {
	let mut cmd = Command::cargo_bin(BIN_NAME)?;

	cmd.arg("--eval='console.log(123);'").assert().success();
	Ok(())
}
