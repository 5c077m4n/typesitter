use std::{
	fs,
	io::{stdin, stdout, BufRead, Write},
	path::PathBuf,
};

use anyhow::Result;
use ast::{parser::parse::Parser as ASTParser, types::node::Node};
use clap::Parser;
use lexer::scanner::scan;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
	filepath: Option<PathBuf>,

	#[clap(short, long)]
	eval: Option<String>,

	#[clap(short, long)]
	check_only: bool,
}

#[cfg(all(feature = "vm", feature = "llvm"))]
compile_error!("The two features `vm` and `llvm` cannot be used at the same time");

#[cfg(all(feature = "vm", not(feature = "llvm")))]
fn start(tree: &Node, check_only: bool) -> Result<()> {
	use bytecode::codegen::CodeGen;
	use vm::vm::VM;

	let mut vm = VM::default();
	let mut codegen = CodeGen::default();

	if !check_only {
		let program = codegen.run(tree)?;
		vm.interpret(&program)?;
	}

	Ok(())
}
#[cfg(all(feature = "llvm", not(feature = "vm")))]
fn start(tree: &Node, check_only: bool) -> Result<()> {
	if !check_only {
		llvm::run(tree)?;
	}

	Ok(())
}

fn main() -> Result<()> {
	env_logger::init();

	let args: Args = Args::parse();
	if let Some(filepath) = args.filepath {
		let input = fs::read(&filepath)?;

		let tokens = scan(&input, filepath.into_os_string().into_string().ok());
		let mut parser = ASTParser::new(tokens);
		let (ast, errors) = parser.parse()?;

		if !errors.is_empty() {
			eprintln!("{:#?}", &errors);
		}

		start(&ast, args.check_only)?;
	} else if let Some(input) = args.eval {
		let input = &input.trim();
		let input = input.as_bytes();

		let tokens = scan(input, Some("Evaluate".to_owned()));
		let mut parser = ASTParser::new(tokens);
		let (ast, errors) = parser.parse()?;

		if !errors.is_empty() {
			eprintln!("{:#?}", errors);
		}

		start(&ast, args.check_only)?;
	} else {
		let mut stdout_handle = stdout().lock();
		let mut stdin_handle = stdin().lock();

		loop {
			print!(">>> ");
			let _ = stdout_handle.flush();

			let mut input = String::new();
			stdin_handle.read_line(&mut input)?;
			let input = input.trim().as_bytes();

			let tokens = scan(input, Some("REPL".to_owned()));
			let mut parser = ASTParser::new(tokens);
			let (ast, errors) = parser.parse()?;

			if !errors.is_empty() {
				eprintln!("{:#?}", errors);
			}

			start(&ast, args.check_only)?;
		}
	}

	Ok(())
}
