use anyhow::Result;
use ast::parser::parse::Parser as ASTParser;
use bytecode::codegen::CodeGen;
use clap::Parser;
use lexer::scanner::scan;
use std::{
	fs,
	io::{stdin, stdout, Read, Write},
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

		let tokens = scan(input, filepath.into_os_string().into_string().ok());
		let mut parser = ASTParser::new(tokens);
		let ast = parser.parse_into_block()?;
		let program = codegen.run(&ast)?;

		vm.interpret(&program)?;
	} else if let Some(input) = args.eval {
		let input = &input.trim();
		let input = input.as_bytes();

		let tokens = scan(input, Some("Evaluate".to_owned()));
		let mut parser = ASTParser::new(tokens);
		let ast = parser.parse_into_block()?;
		let program = codegen.run(&ast)?;

		vm.interpret(&program)?;
	} else {
		loop {
			print!(">>> ");
			let _ = stdout().flush();

			let mut input = vec![];
			let _ = stdin().read(&mut input)?;
			if input == b"\n" {
				break;
			}

			let tokens = scan(&input, Some("REPL".to_owned()));
			let mut parser = ASTParser::new(tokens);
			let ast = parser.parse_into_block()?;
			let program = codegen.run(&ast)?;

			vm.interpret(&program)?;
		}
	}

	Ok(())
}
