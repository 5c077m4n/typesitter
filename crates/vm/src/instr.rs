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
	Get(Pointer),
	Set(Pointer),
}

pub type Program<'p> = &'p [Instr];
