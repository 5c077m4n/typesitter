pub type Pointer = usize;

pub enum Instr {
	Push(f64),
	Pop,
	Add,
	Sub,
	Mul,
	Div,
	Print,
	Jump(Pointer),
	JE(Pointer),
	JNE(Pointer),
}

pub type Program<'p> = &'p [Instr];
