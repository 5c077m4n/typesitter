pub type Pointer = usize;

#[derive(Debug)]
pub enum Instr {
	/// Add a new stack item
	Push(f64),
	/// Get the top stack value and remove it
	Pop,
	/// Sum the two top reg entries together
	AddRegReg,
	/// Add the given number to the last stack entry
	AddRegLit(f64),
	/// Subtract the two top reg entries together
	SubRegReg,
	/// Subtract the given number from the last stack entry
	SubRegLit(f64),
	/// Multiply the two top stack entries together
	MulRegReg,
	/// Multiply the top stack entry with the given number
	MulRegLit(f64),
	/// Divide the top stack entry by the second top
	DivRegReg,
	/// Divide the top stack entry by the given literal
	DivRegLit(f64),
	/// Jump to instruction number
	Jump(Pointer),
	/// Jump to instruction number if stack top is equal to 0
	JumpEqual0(Pointer),
	/// Jump to instruction number if stack top is not equal to 0
	JumpNotEqual0(Pointer),
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
	/// Print top stack item
	Print,
	/// Print top stack item as a char
	PrintChar,
	/// Print the whole stack
	PrintDebug,
}

pub type Program<'p> = &'p [Instr];
