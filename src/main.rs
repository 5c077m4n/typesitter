use anyhow::Result;
use clap::Parser;
use std::io::{stdin, stdout, Write};
use tree_sitter_typescript::language_typescript;

mod lib;

use lib::ast_walker::walk;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
	#[clap(short, long)]
	eval: Option<String>,
}

fn main() -> Result<()> {
	env_logger::init();
	let args: Args = Args::parse();

	let mut parser = tree_sitter::Parser::new();
	parser.set_language(language_typescript()).unwrap();

	if let Some(code) = args.eval {
		let tree = parser
			.parse(&code, None)
			.expect("Couldn't parse the requested code...");
		let mut cursor = tree.walk();
        let split_code: Vec<&str> = code.split("\n").collect();

		walk(&mut cursor, &split_code)?;
	} else {
		loop {
			print!(">>> ");
			let _ = stdout().flush();

			let mut input = String::new();
			stdin().read_line(&mut input)?;
			if input == "\n" {
				break;
			}

			let tree = parser
				.parse(&input, None)
				.expect("Couldn't parse the inputted code");
			let mut cursor = tree.walk();

            let split_code: Vec<&str> = input.split("\n").collect();
			walk(&mut cursor, &split_code)?;
		}
	}

	Ok(())
}
