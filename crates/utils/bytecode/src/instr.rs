pub type Pointer = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
	/// Add a new stack item
	Push(f64),
	/// Get the top stack value and remove it
	Pop,
	/// Sum the two top reg entries together
	Add,
	/// Add the given number to the last stack entry
	AddLit(f64),
	/// Add two registers and put the result in the third
	AddRegReg {
		reg_1: Pointer,
		reg_2: Pointer,
		reg_result: Pointer,
	},
	/// Add register and literal and put the result in the third
	AddRegLit {
		reg_1: Pointer,
		value: f64,
		reg_result: Pointer,
	},
	/// Subtract the two top reg entries together
	Sub,
	/// Subtract the given number from the last stack entry
	SubLit(f64),
	/// Subtract two registers and put the result in the third
	SubRegReg {
		reg_1: Pointer,
		reg_2: Pointer,
		reg_result: Pointer,
	},
	/// Subtract register and literal and put the result in the third
	SubRegLit {
		reg_1: Pointer,
		value: f64,
		reg_result: Pointer,
	},
	/// Multiply the two top stack entries together
	Mul,
	/// Multiply the top stack entry with the given number
	MulLit(f64),
	/// Mutliply two registers and put the result in the third
	MulRegReg {
		reg_1: Pointer,
		reg_2: Pointer,
		reg_result: Pointer,
	},
	/// Multiply register and literal and put the result in the third
	MulRegLit {
		reg_1: Pointer,
		value: f64,
		reg_result: Pointer,
	},
	/// Divide the top stack entry by the second top
	Div,
	/// Divide the top stack entry by the given literal
	DivLit(f64),
	/// Divide two registers and put the result in the third
	DivRegReg {
		reg_1: Pointer,
		reg_2: Pointer,
		reg_result: Pointer,
	},
	/// Divide register and literal and put the result in the third
	DivRegLit {
		reg_1: Pointer,
		value: f64,
		reg_result: Pointer,
	},
	/// Jump to instruction number
	Jump(Pointer),
	/// Jump to instruction number if stack top is equal to 0
	JumpEqual { ip: Pointer, value: f64 },
	/// Jump to instruction number if stack top is not equal to 0
	JumpNotEqual { ip: Pointer, value: f64 },
	/// Get value at stack item
	Get(Pointer),
	/// Set value at stack item
	Set(Pointer),
	/// Get an arg from before the fn start
	GetArg(Pointer),
	/// Set an arg from before the fn start
	SetArg(Pointer),
	/// Call a function
	Call(Pointer),
	/// Return from a function
	Ret,
	/// Print top stack item into stdout
	PrintOut,
	/// Print top stack item as a char into stdout
	PrintChar,
	/// Print top stack item into stderr
	PrintErr,
	/// Read from stdin
	Read,
	/// Print the whole stack
	PrintDebug,
}

pub type Program = Vec<Instr>;
