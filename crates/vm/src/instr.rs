pub type Pointer = usize;

pub enum Instr {
	Push(f64),
	Pop,
	Add,
	Sub,
	Incr,
	Decr,
	Mul,
	Div,
	Jump(Pointer),
	JumpEqual0(Pointer),
	JumpNotEqual0(Pointer),
	Get(Pointer),
	Set(Pointer),
	Print,
	PrintChar,
	PrintDebug,
}

pub type Program<'p> = &'p [Instr];
