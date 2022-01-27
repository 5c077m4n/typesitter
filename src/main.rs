use anyhow::Result;
use clap::Parser;
use std::io::{stdin, stdout, Write};

mod lib;

use lib::tokenizer::scanner::scan;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
	#[clap(short, long)]
	eval: Option<String>,
}

fn main() -> Result<()> {
	env_logger::init();
	let args: Args = Args::parse();

	if let Some(input) = args.eval {
		let input = &input.trim();
		let _tokens = scan(input, Some("Evaluation script".to_owned())).collect::<Vec<_>>();
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
			let _tokens = scan(input, Some("REPL".to_owned())).collect::<Vec<_>>();
		}
	}

	Ok(())
}
