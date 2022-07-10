use anyhow::{anyhow, Result};
use bytecode::instr::Pointer;

#[derive(Debug, Default)]
pub struct StackFrame {
	/// Stack pointer
	pub sp: Pointer,
	/// Instruction pointer
	pub ip: Pointer,
}

#[derive(Debug)]
pub struct CallStack(Vec<StackFrame>);

impl Default for CallStack {
	fn default() -> Self {
		Self(Vec::with_capacity(1024))
	}
}
impl CallStack {
	#[allow(dead_code)]
	pub fn new(capacity: usize) -> Self {
		Self(Vec::with_capacity(capacity))
	}
	pub fn push(&mut self, sf: StackFrame) {
		self.0.push(sf)
	}
	pub fn pop(&mut self) -> Result<StackFrame> {
		self.0
			.pop()
			.ok_or_else(|| anyhow!("The call stack is empty"))
	}
	pub fn last_offset(&self) -> usize {
		self.0.last().map_or(0, |sf| sf.sp)
	}
	pub fn get_offset(&self) -> Result<usize> {
		let first_index_before_fn = self
			.0
			.last()
			.ok_or_else(|| anyhow!("The call stack is empty"))?
			.sp - 1;
		Ok(first_index_before_fn)
	}
}
