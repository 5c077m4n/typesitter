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
	JumpEqual(Pointer),
	JumpNotEqual(Pointer),
	Get(Pointer),
	Set(Pointer),
	Print,
	PrintChar,
}

pub type Program<'p> = &'p [Instr];
