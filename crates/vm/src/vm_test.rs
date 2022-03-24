use super::{instr::Instr, vm::VM};

#[test]
fn simple_arith_one_plus_one() {
	use Instr::*;

	let mut vm = VM::default();
	let result = vm.interpret(&[Push(1.), Push(1.), Instr::Add]);

	assert_eq!(result, Some(2.));
}

#[test]
fn complex_arith() {
	use Instr::*;

	let mut vm = VM::default();
	let result = vm.interpret(&[Push(9.), Push(3.), Push(1.), Add, Sub]);

	assert_eq!(result, Some(5.));
}

#[test]
#[ignore]
fn sum_first_100_ints() {
	use Instr::*;

	let mut vm = VM::default();
	let result = vm.interpret(&[
		Push(0.), // [accumilator = 0]
		Push(1.), // [accumilator = 0, index = 1]
		// stack: [accumilator, index]
		Get(0), // [accumilator, index, accumilator] <- Loop start - jump to here
		Get(1), // [accumilator, index, accumilator, index]
		Add,    // [accumilator, index, (accumilator + index)]
		Set(0), // [accumilator + (accumilator + index), index, (accumilator + index)]
		Pop,    // [accumilator + (accumilator + index), index]
		// stack: [accumilator, index]
		Incr,             // [accumilator, index]
		Get(1),           // [accumilator, index, index]
		Push(100.),       // [accumilator, index, index, 100]
		Sub,              // [accumilator, index, index - 100]
		JumpNotEqual0(2), // [accumilator, index] <- Loop end
		Get(0),           // [accumilator, index, accumilator]
		Print,
	]);

	assert_eq!(result, Some(100.));
}
