pub type Pointer = usize;

#[derive(Debug)]
pub enum Instr {
	/// Add a new stack item
	Push(f64),
	/// Get the top stack value and remove it
	Pop,
	/// Sum the two top reg entries together
	Add,
	/// Add the given number to the last stack entry
	AddLit(f64),
	/// Subtract the two top reg entries together
	Sub,
	/// Subtract the given number from the last stack entry
	SubLit(f64),
	/// Multiply the two top stack entries together
	Mul,
	/// Multiply the top stack entry with the given number
	MulLit(f64),
	/// Divide the top stack entry by the second top
	Div,
	/// Divide the top stack entry by the given literal
	DivLit(f64),
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

pub type Program<'p> = &'p [Instr];
