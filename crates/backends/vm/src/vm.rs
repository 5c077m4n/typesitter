use super::{
	call_stack::{CallStack, StackFrame},
	instr::{Instr, Pointer, Program},
	stack::Stack,
};
use anyhow::Result;
use log::debug;
use std::io;

pub struct VM {
	stack: Stack,
	call_stack: CallStack,
	/// Instruction pointer
	ip: Pointer,

	reader: Box<dyn io::Read>,
	writer_out: Box<dyn io::Write>,
	writer_err: Box<dyn io::Write>,
}
impl Default for VM {
	fn default() -> Self {
		Self {
			stack: Stack::default(),
			call_stack: CallStack::default(),
			ip: 0,

			reader: Box::new(io::stdin()),
			writer_out: Box::new(io::stdout()),
			writer_err: Box::new(io::stderr()),
		}
	}
}
impl VM {
	pub fn new(
		reader: Box<dyn io::Read>,
		writer_out: Box<dyn io::Write>,
		writer_err: Box<dyn io::Write>,
	) -> Self {
		Self {
			reader,
			writer_out,
			writer_err,
			..Self::default()
		}
	}

	fn handle_instr(&mut self, instr: &Instr) -> Result<()> {
		match instr {
			Instr::Push(n) => {
				self.stack.push(*n);
			}
			Instr::Pop => {
				let tmp = self.stack.pop()?;
				debug!("{:?}", tmp);
			}
			Instr::Add => {
				let (a, b) = (self.stack.pop()?, self.stack.pop()?);
				self.stack.push(b + a);
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::AddLit(lit) => {
				*self.stack.peek_mut()? += lit;
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::Sub => {
				let (a, b) = (self.stack.pop()?, self.stack.pop()?);
				self.stack.push(b - a);
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::SubLit(lit) => {
				*self.stack.peek_mut()? -= lit;
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::Mul => {
				let (a, b) = (self.stack.pop()?, self.stack.pop()?);
				self.stack.push(b * a);
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::MulLit(lit) => {
				*self.stack.peek_mut()? -= lit;
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::Div => {
				let (a, b) = (self.stack.pop()?, self.stack.pop()?);
				self.stack.push(b / a);
				debug!("{:?}", self.stack.peek()?);
			}
			Instr::DivLit(lit) => {
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
					sp: self.stack.len(),
					ip: self.ip,
				});
				self.ip = *ip;
			}
			Instr::Ret => {
				self.ip = self.call_stack.pop()?.ip;
			}
			Instr::Read => {
				let mut buffer: Vec<u8> = Vec::new();
				self.reader.read_to_end(&mut buffer)?;
				// TODO: do something with the buffer
			}
			Instr::PrintOut => {
				let peek_out = format!("{:?}", *self.stack.peek()?);
				let peek_out = peek_out.as_bytes();
				self.writer_out.write_all(peek_out)?;
			}
			Instr::PrintChar => {
				let peek_out = format!("{:?}", *self.stack.peek()? as u8 as char);
				let peek_out = peek_out.as_bytes();
				self.writer_out.write_all(peek_out)?;
			}
			Instr::PrintErr => {
				let peek_out = format!("{:?}", *self.stack.peek()?);
				let peek_out = peek_out.as_bytes();
				self.writer_err.write_all(peek_out)?;
			}
			Instr::PrintDebug => self.stack.debug(),
		}
		Ok(())
	}
	pub fn interpret(&mut self, program: Program) -> Result<f64> {
		self.ip = 0;
		while let Some(instr) = program.get(self.ip) {
			self.ip += 1;
			self.handle_instr(instr)?;
		}
		self.stack.pop()
	}
}
