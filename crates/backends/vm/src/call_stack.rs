use super::instr::Pointer;
use anyhow::{anyhow, Result};

#[derive(Debug, Default)]
pub struct StackFrame {
	pub stack_offset: Pointer,
	pub ip: Pointer,
}

#[derive(Debug, Default)]
pub struct CallStack(Vec<StackFrame>);
impl CallStack {
	pub fn new() -> Self {
		Self(Vec::with_capacity(1024))
	}
	pub fn push(&mut self, sf: StackFrame) {
		self.0.push(sf)
	}
	pub fn pop(&mut self) -> Result<StackFrame> {
		self.0.pop().ok_or_else(|| {
			anyhow!("There was an error in getting the last element in the call stack")
		})
	}
	pub fn last_offset(&self) -> usize {
		self.0.last().map_or(0, |sf| sf.stack_offset)
	}
	pub fn get_offset(&self) -> Result<usize> {
		let first_index_before_fn =
			self.0
				.last()
				.ok_or_else(|| anyhow!("The stack is empty"))?
				.stack_offset - 1;
		Ok(first_index_before_fn)
	}
}
