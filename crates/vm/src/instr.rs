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
	Print,
	Jump(Pointer),
	JumpEqual(Pointer),
	JumpNotEqual(Pointer),
	Get(Pointer),
	Set(Pointer),
}

pub type Program<'p> = &'p [Instr];
