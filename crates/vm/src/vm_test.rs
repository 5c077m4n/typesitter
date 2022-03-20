use super::{instr::Instr, vm::VM};
use anyhow::Result;

#[test]
fn sanity() -> Result<()> {
	let mut vm = VM::default();
	let result = vm.interpret(&[
		Instr::Push(9.),
		Instr::Push(3.),
		Instr::Push(1.),
		Instr::Add,
		Instr::Sub,
	])?;

	assert_eq!(result, 5.);
	Ok(())
}
