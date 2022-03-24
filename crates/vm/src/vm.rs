use super::{
	instr::{Instr, Pointer, Program},
	stack::Stack,
};
use log::debug;

#[derive(Default, Debug)]
pub struct VM {
	stack: Stack,
	/// Instruction pointer
	ip: Pointer,
}
impl VM {
	pub fn handle_instr(&mut self, instr: &Instr) {
		match instr {
			Instr::Push(n) => {
				self.stack.push(*n);
			}
			Instr::Pop => {
				let tmp = self.stack.pop();
				debug!("{:?}", tmp);
			}
			Instr::Add => {
				if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
					self.stack.push(b + a);
				}
				debug!("{:?}", self.stack.peek());
			}
			Instr::Incr => *self.stack.peek_mut().unwrap() += 1.,
			Instr::Sub => {
				if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
					self.stack.push(b - a);
				}
				debug!("{:?}", self.stack.peek());
			}
			Instr::Decr => *self.stack.peek_mut().unwrap() -= 1.,
			Instr::Mul => {
				if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
					self.stack.push(b * a);
				}
				debug!("{:?}", self.stack.peek());
			}
			Instr::Div => {
				if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
					self.stack.push(b / a);
				}
				debug!("{:?}", self.stack.peek());
			}
			Instr::Jump(ip) => {
				self.ip = *ip;
			}
			Instr::JumpEqual(ip) => {
				if self.stack.peek() == Some(&0.) {
					self.ip = *ip;
				}
			}
			Instr::JumpNotEqual(ip) => {
				if self.stack.peek() != Some(&0.) {
					self.ip = *ip;
				}
			}
			Instr::Get(ip) => {
				if let Some(&v) = self.stack.get(*ip) {
					self.stack.push(v);
				}
			}
			Instr::Set(ip) => {
				if let Some(v) = self.stack.pop() {
					let val = self.stack.get_mut(*ip).unwrap();
					*val = v;
				}
			}
			Instr::Print => {
				println!("{:?}", self.stack.peek().unwrap());
			}
			Instr::PrintChar => {
				println!("{:?}", *self.stack.peek().unwrap() as u8 as char);
			}
		}
	}
	pub fn interpret(&mut self, program: Program) -> Option<f64> {
		self.ip = 0;

		while let Some(instr) = program.get(self.ip) {
			self.handle_instr(&instr);
			self.ip += 1;
		}

		self.stack.pop()
	}
}
