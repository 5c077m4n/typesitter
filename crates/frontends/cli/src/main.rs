use anyhow::Result;
use ast::parser::parse::parse_into_block;
use bytecode::codegen::CodeGen;
use clap::Parser;
use lexer::scanner::scan;
use std::{
	fs,
	io::{stdin, stdout, Write},
	path::PathBuf,
};
use vm::vm::VM;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
	filepath: Option<PathBuf>,

	#[clap(short, long)]
	eval: Option<String>,
}

fn main() -> Result<()> {
	env_logger::init();
	let args: Args = Args::parse();
	let mut vm = VM::default();
	let mut codegen = CodeGen::default();

	if let Some(filepath) = args.filepath {
		let input = fs::read_to_string(&filepath)?;
		let input = &input.trim();
		let input = input.as_bytes();

		let mut tokens = scan(input, filepath.into_os_string().into_string().ok());
		let ast = parse_into_block(&mut tokens)?;
		let program = codegen.run(&ast)?;

		if !program.is_empty() {
			vm.interpret(&program)?;
		}
	} else if let Some(input) = args.eval {
		let input = &input.trim();
		let input = input.as_bytes();

		let mut tokens = scan(input, Some("Evaluate".to_owned()));
		let ast = parse_into_block(&mut tokens)?;
		let program = codegen.run(&ast)?;

		if !program.is_empty() {
			vm.interpret(&program)?;
		}
	} else {
		loop {
			print!(">>> ");
			let _ = stdout().flush();

			let mut input = String::new();
			stdin().read_line(&mut input)?;
			if input == "\n" {
				break;
			}

			let input = &input.trim();
			let input = input.as_bytes();
			let mut tokens = scan(input, Some("REPL".to_owned()));
			let ast = parse_into_block(&mut tokens)?;
			let program = codegen.run(&ast)?;

			if !program.is_empty() {
				vm.interpret(&program)?;
			}
		}
	}

	Ok(())
}
