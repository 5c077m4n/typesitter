use anyhow::Result;
use clap::Parser;
use std::{
	fs,
	io::{stdin, stdout, Write},
	path::PathBuf,
};

mod lib;

use lib::tokenizer::scanner::scan;

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

	if let Some(filepath) = args.filepath {
		let input = fs::read_to_string(filepath)?;
		let input = &input.trim();

		let _tokens: Vec<_> = scan(input, Some("Evaluation script".to_owned())).collect();
	} else if let Some(input) = args.eval {
		let input = &input.trim();
		let _tokens: Vec<_> = scan(input, Some("Evaluation script".to_owned())).collect();
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
			let _tokens: Vec<_> = scan(input, Some("REPL".to_owned())).collect();
		}
	}

	Ok(())
}
