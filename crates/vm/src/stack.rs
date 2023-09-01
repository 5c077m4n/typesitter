use super::instr::Pointer;

#[derive(Default, Debug)]
pub struct Stack(Vec<f64>);

impl Stack {
	pub fn push(&mut self, v: f64) {
		self.0.push(v)
	}
	pub fn get(&self, p: Pointer) -> Option<&f64> {
		self.0.get(p)
	}
	pub fn get_mut(&mut self, p: Pointer) -> Option<&mut f64> {
		self.0.get_mut(p)
	}
	pub fn peek(&mut self) -> Option<&f64> {
		self.0.last()
	}
	pub fn peek_mut(&mut self) -> Option<&mut f64> {
		self.0.last_mut()
	}
	pub fn pop(&mut self) -> Option<f64> {
		self.0.pop()
	}
	#[allow(dead_code)]
	pub(crate) fn debug(&self) {
		println!("{:#?}", self.0);
	}
}
