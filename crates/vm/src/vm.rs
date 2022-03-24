use super::{
	instr::{Instr, Pointer, Program},
	stack::Stack,
};
use anyhow::Result;
use log::debug;

#[derive(Default, Debug)]
pub struct VM {
	stack: Stack,
	/// Instruction pointer
	ip: Pointer,
}
impl VM {
	pub fn handle_instr(&mut self, instr: &Instr) -> Result<()> {
		match instr {
			Instr::Push(n) => {
				self.stack.push(*n);
			}
			Instr::Pop => {
				let tmp = self.stack.pop();
				debug!("{:?}", tmp);
			}
			Instr::AddRegReg => {
				let (a, b) = (self.stack.pop()?, self.stack.pop()?);
				self.stack.push(b + a);
				debug!("{:?}", self.stack.peek());
			}
			Instr::AddRegLit(lit) => {
				*self.stack.peek_mut().unwrap() += lit;
				debug!("{:?}", self.stack.peek());
			}
			Instr::SubRegReg => {
				let (a, b) = (self.stack.pop()?, self.stack.pop()?);
				self.stack.push(b - a);
				debug!("{:?}", self.stack.peek());
			}
			Instr::SubRegLit(lit) => {
				*self.stack.peek_mut().unwrap() -= lit;
				debug!("{:?}", self.stack.peek());
			}
			Instr::MulRegReg => {
				let (a, b) = (self.stack.pop()?, self.stack.pop()?);
				self.stack.push(b * a);
				debug!("{:?}", self.stack.peek());
			}
			Instr::MulRegLit(lit) => {
				*self.stack.peek_mut().unwrap() -= lit;
				debug!("{:?}", self.stack.peek());
			}
			Instr::DivRegReg => {
				let (a, b) = (self.stack.pop()?, self.stack.pop()?);
				self.stack.push(b / a);
				debug!("{:?}", self.stack.peek());
			}
			Instr::DivRegLit(lit) => {
				*self.stack.peek_mut().unwrap() -= lit;
				debug!("{:?}", self.stack.peek());
			}
			Instr::Jump(ip) => {
				self.ip = *ip;
			}
			Instr::JumpEqual0(ip) => {
				if *self.stack.peek()? == 0. {
					self.stack.pop()?;
					self.ip = *ip;
				}
			}
			Instr::JumpNotEqual0(ip) => {
				if *self.stack.peek()? != 0. {
					self.stack.pop()?;
					self.ip = *ip;
				}
			}
			Instr::Get(ip) => {
				let value = *self.stack.get(*ip)?;
				self.stack.push(value);
			}
			Instr::Set(ip) => {
				let last_value = self.stack.pop()?;
				*self.stack.get_mut(*ip)? = last_value;
			}
			Instr::Print => println!("{:?}", *self.stack.peek()?),
			Instr::PrintChar => println!("{:?}", *self.stack.peek()? as u8 as char),
			Instr::PrintDebug => self.stack.debug(),
		}
		Ok(())
	}
	pub fn interpret(&mut self, program: Program) -> Result<f64> {
		self.ip = 0;

		while let Some(instr) = program.get(self.ip) {
			self.handle_instr(&instr)?;
			self.ip += 1;
		}

		self.stack.pop()
	}
}
