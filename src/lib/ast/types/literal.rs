#[derive(Debug)]
pub enum Literal<'l> {
	Undefined,
	Null,
	String(&'l str),
	Number(f64),
	Bool(bool),
}
