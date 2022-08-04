use super::{
	call_stack::{CallStack, StackFrame},
	register_file::RegisterFile,
	stack::Stack,
};
use anyhow::Result;
use bytecode::instr::{Instr, Pointer, Program};
use log::debug;
use std::io;

type R = Box<dyn io::Read>;
type W = Box<dyn io::Write>;

pub struct VM {
	stack: Stack,
	call_stack: CallStack,
	/// Instruction pointer
	ip: Pointer,

	register_file: RegisterFile,

	reader: R,
	writer_out: W,
	writer_err: W,
}
impl Default for VM {
	fn default() -> Self {
		Self {
			stack: Stack::default(),
			call_stack: CallStack::default(),
			ip: 0,

			register_file: RegisterFile::default(),

			reader: Box::new(io::stdin()),
			writer_out: Box::new(io::stdout()),
			writer_err: Box::new(io::stderr()),
		}
	}
}
impl VM {
	pub fn new(reader: R, writer_out: W, writer_err: W) -> Self {
		Self {
			reader,
			writer_out,
			writer_err,
			..Self::default()
		}
	}

	fn call_builtin(&mut self, fn_name: &str, params: &[Pointer]) -> Result<()> {
		match fn_name {
			"console.log" => {
				let params: Vec<String> = params
					.iter()
					.map(|pointer| {
						let value = self.stack.get(*pointer).unwrap();
						format!("{}", &value)
					})
					.collect();
				self.writer_out
					.write_fmt(format_args!("{}\n", params.join(", ")))?;
			}
			"console.error" => {
				let params: Vec<String> = params
					.iter()
					.map(|pointer| {
						let value = self.stack.get(*pointer).unwrap();
						format!("{}", &value)
					})
					.collect();
				self.writer_err
					.write_fmt(format_args!("{}\n", params.join(", ")))?;
			}
			other => {
				todo!("Support other builtins `{:?}`", other)
			}
		}
		Ok(())
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
			Instr::AddRegReg {
				reg_1,
				reg_2,
				reg_result,
			} => {
				self.register_file[*reg_result] =
					self.register_file[*reg_1] + self.register_file[*reg_2];
			}
			Instr::AddRegLit {
				reg_1,
				value,
				reg_result,
			} => {
				self.register_file[*reg_result] = self.register_file[*reg_1] + *value;
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
			Instr::SubRegReg {
				reg_1,
				reg_2,
				reg_result,
			} => {
				self.register_file[*reg_result] =
					self.register_file[*reg_1] - self.register_file[*reg_2];
			}
			Instr::SubRegLit {
				reg_1,
				value,
				reg_result,
			} => {
				self.register_file[*reg_result] = self.register_file[*reg_1] - *value;
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
			Instr::MulRegReg {
				reg_1,
				reg_2,
				reg_result,
			} => {
				self.register_file[*reg_result] =
					self.register_file[*reg_1] * self.register_file[*reg_2];
			}
			Instr::MulRegLit {
				reg_1,
				value,
				reg_result,
			} => {
				self.register_file[*reg_result] = self.register_file[*reg_1] * *value;
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
			Instr::DivRegReg {
				reg_1,
				reg_2,
				reg_result,
			} => {
				self.register_file[*reg_result] =
					self.register_file[*reg_1] / self.register_file[*reg_2];
			}
			Instr::DivRegLit {
				reg_1,
				value,
				reg_result,
			} => {
				self.register_file[*reg_result] = self.register_file[*reg_1] / *value;
			}
			Instr::Jump(ip) => {
				self.ip = *ip;
			}
			Instr::JumpEqual { ip, value } => {
				if self.stack.pop()? == *value {
					self.ip = *ip;
				}
			}
			Instr::JumpNotEqual { ip, value } => {
				if self.stack.pop()? != *value {
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
			Instr::CallBuiltin(fn_name, params) => {
				self.call_builtin(fn_name, params)?;
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
	pub fn interpret(&mut self, program: &Program) -> Result<&f64> {
		while let Some(instr) = program.get(self.ip) {
			self.ip += 1;
			self.handle_instr(instr)?;
		}
		self.stack.last()
	}
}
