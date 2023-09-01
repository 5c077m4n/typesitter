use super::{
	instr::{Instr, Program},
	stack::Stack,
};
use anyhow::Result;
use log::debug;

#[derive(Default, Debug)]
pub struct VM {
	stack: Stack,
	/// Instruction pointer
	ip: usize,
}
impl VM {
	pub fn interpret(&mut self, program: Program) -> Result<f64> {
		self.ip = 0;

		while let Some(instr) = program.get(self.ip) {
			match instr {
				Instr::Push(n) => self.stack.push(*n),
				Instr::Pop => {
					let tmp = self.stack.pop()?;
					debug!("{:?}", tmp);
				}
				Instr::Add => {
					let (a, b) = (self.stack.pop()?, self.stack.pop()?);
					self.stack.push(b + a);

					debug!("{:?}", self.stack.peek()?);
				}
				Instr::Sub => {
					let (a, b) = (self.stack.pop()?, self.stack.pop()?);
					self.stack.push(b - a);

					debug!("{:?}", self.stack.peek()?);
				}
				Instr::Mul => {
					let (a, b) = (self.stack.pop()?, self.stack.pop()?);
					self.stack.push(b * a);

					debug!("{:?}", self.stack.peek()?);
				}
				Instr::Div => {
					let (a, b) = (self.stack.pop()?, self.stack.pop()?);
					self.stack.push(b / a);

					debug!("{:?}", self.stack.peek()?);
				}
				Instr::Print => {
					println!("{:?}", self.stack.peek()?);
				}
				Instr::Jump(p) => {
					self.ip = *p;
				}
				Instr::JE(p) => {
					if *self.stack.peek()? == 0. {
						self.ip = *p;
					}
				}
				Instr::JNE(p) => {
					if *self.stack.peek()? != 0. {
						self.ip = *p;
					}
				}
			}
			self.ip += 1;
		}

		self.stack.pop()
	}
}
