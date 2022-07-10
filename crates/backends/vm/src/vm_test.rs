use super::{instr::Instr, vm::VM};
use anyhow::Result;
use macros::test_with_logger;

#[test_with_logger]
fn simple_arith_one_plus_one() -> Result<()> {
	use Instr::*;

	let result = VM::default().interpret(&[Push(1.), Push(1.), Add])?;
	assert_eq!(result, 2.);
	Ok(())
}

#[test_with_logger]
fn complex_arith() -> Result<()> {
	use Instr::*;

	let result = VM::default().interpret(&[Push(9.), Push(3.), Push(1.), Add, Sub])?;
	assert_eq!(result, 5.);
	Ok(())
}

#[test_with_logger]
fn sum_first_100_ints() -> Result<()> {
	use Instr::*;

	let result = VM::default().interpret(&[
		Push(0.), // [accumilator = 0]
		Push(0.), // [accumilator = 0, index = 0]
		// stack: [accumilator, index]
		Get(0), // [accumilator, index, accumilator] <- Loop start - jump to here
		Get(1), // [accumilator, index, accumilator, index]
		Add,    // [accumilator, index, (accumilator + index)]
		Set(0), // [(accumilator + index), index]
		// stack: [accumilator, index]
		AddLit(1.),       // [accumilator, index + 1]
		Get(1),           // [accumilator, index, index]
		SubLit(100.),     // [accumilator, index, index - 100]
		JumpNotEqual0(2), // [accumilator, index] <- Loop end
		Get(0),           // [accumilator, index, accumilator]
	])?;
	let sum_upto_100 = (0..100).sum::<usize>() as f64;

	assert_eq!(result, sum_upto_100);
	Ok(())
}
