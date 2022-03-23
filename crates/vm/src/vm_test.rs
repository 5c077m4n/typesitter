use super::{instr::Instr, vm::VM};

#[test]
fn simple_calc() {
	let mut vm = VM::default();
	let result = vm.interpret(&[
		Instr::Push(9.),
		Instr::Push(3.),
		Instr::Push(1.),
		Instr::Add,
		Instr::Sub,
	]);

	assert_eq!(result, Some(5.));
}

#[test]
#[ignore]
fn sum_first_100_ints() {
	let mut vm = VM::default();
	let result = vm.interpret(&[
		// setup
		Instr::Push(0.), // the accumulator
		Instr::Push(0.), // the index
		// loop
		// First, add the index to the accumulator
		// stack: [accumulator, index]
		Instr::Get(0),
		Instr::Get(1),
		// stack: [accumulator, index, accumulator, index]
		Instr::Add,
		// stack: [accumulator, index, accumulator + index]
		Instr::Set(0),
		Instr::Pop,
		// stack: [accumulator + index, index]

		// next, increment the index
		Instr::Push(1.), // the increment
		// stack: [accumulator, index, 1]
		Instr::Add,
		// stack: [accumulator, index + 1]

		// finally, compare the index with 100 and jump back to the start
		// if they're not equal.
		Instr::Get(1),
		// stack: [accumulator, index, index]
		Instr::Push(100.),
		Instr::Sub,
		// stack: [accumulator, index, index - 100]
		Instr::JumpNotEqual(2),
		// if index - 100 == 0, print the accumulator
		Instr::Get(0),
		// stack: [accumulator, index, 0, accumulator]
	]);

	assert_eq!(result, Some(100.));
}
