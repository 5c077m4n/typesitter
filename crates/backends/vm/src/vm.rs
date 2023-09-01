use super::{
	call_stack::{CallStack, StackFrame},
	instr::{Instr, Pointer, Program},
	stack::Stack,
};
use anyhow::Result;
use log::debug;

#[derive(Debug, Default)]
pub struct VM {
	stack: Stack,
	call_stack: CallStack,
	/// Instruction pointer
	ip: Pointer,
}
impl VM {
	fn handle_instr(&mut self, instr: &Instr) -> Result<()> {
		match instr {
			Instr::Push(n) => {
				self.stack.push(*n);
			}
			Instr::Pop => {
				let tmp = self.stack.pop()?;
				debug!("{:?}", tmp);
			}
			Instr::AddRegReg => {
				let (a, b) = (self.stack.pop()?, self.stack.pop()?);
				self.stack.push(b + a);
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::AddRegLit(lit) => {
				*self.stack.peek_mut()? += lit;
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::SubRegReg => {
				let (a, b) = (self.stack.pop()?, self.stack.pop()?);
				self.stack.push(b - a);
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::SubRegLit(lit) => {
				*self.stack.peek_mut()? -= lit;
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::MulRegReg => {
				let (a, b) = (self.stack.pop()?, self.stack.pop()?);
				self.stack.push(b * a);
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::MulRegLit(lit) => {
				*self.stack.peek_mut()? -= lit;
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::DivRegReg => {
				let (a, b) = (self.stack.pop()?, self.stack.pop()?);
				self.stack.push(b / a);
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::DivRegLit(lit) => {
				*self.stack.peek_mut()? -= lit;
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::Jump(ip) => {
				self.ip = *ip;
			}
			Instr::JumpEqual0(ip) => {
				if self.stack.pop()? == 0. {
					self.ip = *ip;
				}
			}
			Instr::JumpNotEqual0(ip) => {
				if self.stack.pop()? != 0. {
					self.ip = *ip;
				}
			}
			Instr::Get(ip) => {
				let value = *self.stack.get(*ip + self.call_stack.last_offset())?;
				self.stack.push(value);
			}
			Instr::Set(ip) => {
				let last_value = self.stack.pop()?;
				let offset = *ip + self.call_stack.last_offset();
				*self.stack.get_mut(offset)? = last_value;
			}
			Instr::GetArg(i) => {
				let offset = self.call_stack.get_offset()? - *i;
				let arg = *self.stack.get(offset)?;
				self.stack.push(arg);
			}
			Instr::SetArg(i) => {
				let offset_i = self.call_stack.get_offset()? - *i;
				let last_val = *self.stack.peek()?;
				*self.stack.get_mut(offset_i)? = last_val;
			}
			Instr::Call(ip) => {
				self.call_stack.push(StackFrame {
					stack_offset: self.stack.len(),
					ip: self.ip,
				});
				self.ip = *ip;
			}
			Instr::Ret => {
				self.ip = self.call_stack.pop()?.ip;
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
			self.ip += 1;
			self.handle_instr(&instr)?;
		}
		self.stack.pop()
	}
}
