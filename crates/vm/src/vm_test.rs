use super::{instr::Instr, vm::VM};

#[test]
fn simple_calc_one_plus_one() {
	let mut vm = VM::default();
	let result = vm.interpret(&[Instr::Push(1.), Instr::Push(1.), Instr::Add]);

	assert_eq!(result, Some(2.));
}

#[test]
fn complex_arith() {
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
		Instr::Push(0.), // [accumilator = 0]
		Instr::Push(0.), // [accumilator = 0, index = 0]
	]);

	assert_eq!(result, Some(100.));
}
