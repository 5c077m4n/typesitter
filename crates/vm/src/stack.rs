use anyhow::{bail, Result};

#[derive(Default, Debug)]
pub struct Stack(Vec<f64>);
impl Stack {
	pub fn push(&mut self, v: f64) {
		self.0.push(v)
	}
	pub fn pop(&mut self) -> Result<f64> {
		if let Some(n) = self.0.pop() {
			Ok(n)
		} else {
			bail!(
				"There was an error in getting the stack's pop: {:?}",
				self.0
			);
		}
	}
	pub fn peek(&mut self) -> Result<&f64> {
		if let Some(n) = self.0.last() {
			Ok(n)
		} else {
			bail!(
				"There was an error in getting the stack's pop: {:?}",
				self.0
			);
		}
	}
	pub fn peek_mut(&mut self) -> Result<&mut f64> {
		if let Some(n) = self.0.last_mut() {
			Ok(n)
		} else {
			bail!("There was an error in getting the stack's pop");
		}
	}
}
