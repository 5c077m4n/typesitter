use anyhow::{anyhow, Result};
use bytecode::instr::Pointer;

#[derive(Debug)]
pub struct Stack(Vec<f64>);

impl Default for Stack {
	fn default() -> Self {
		Self(Vec::with_capacity(1024))
	}
}
impl Stack {
	#[allow(dead_code)]
	pub fn new(capacity: usize) -> Self {
		Self(Vec::with_capacity(capacity))
	}
	pub fn push(&mut self, v: f64) {
		self.0.push(v);
	}
	pub fn get(&self, p: Pointer) -> Result<&f64> {
		self.0
			.get(p)
			.ok_or_else(|| anyhow!("There should be an {}th item in the stack", p))
	}
	pub fn get_mut(&mut self, p: Pointer) -> Result<&mut f64> {
		self.0
			.get_mut(p)
			.ok_or_else(|| anyhow!("There should be a mutable {}th item in the stack", p))
	}
	pub fn peek(&self) -> Result<&f64> {
		self.0
			.last()
			.ok_or_else(|| anyhow!("There should be a last item in the stack"))
	}
	pub fn peek_mut(&mut self) -> Result<&mut f64> {
		self.0
			.last_mut()
			.ok_or_else(|| anyhow!("There should be a last mutable item in the stack"))
	}
	pub fn pop(&mut self) -> Result<f64> {
		self.0
			.pop()
			.ok_or_else(|| anyhow!("There should be a last item on the stack"))
	}
	pub fn last(&mut self) -> Result<&f64> {
		self
			.0
			.last()
			.ok_or_else(|| anyhow!("There should be a last item on the stack"))
	}
	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub(crate) fn debug(&self) {
		println!("{:#?}", self.0);
	}
}
