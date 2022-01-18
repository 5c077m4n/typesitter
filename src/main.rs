use anyhow::Result;
use clap::Parser;
use log::debug;
use std::io::{stdin, stdout, Write};

mod lib;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
	#[clap(short, long)]
	eval: Option<String>,
}

fn main() -> Result<()> {
	env_logger::init();
	let args: Args = Args::parse();

	if let Some(code) = args.eval {
        debug!("{code}");
	} else {
		loop {
			print!(">>> ");
			let _ = stdout().flush();

			let mut input = String::new();
			stdin().read_line(&mut input)?;
			if input == "\n" {
				break;
			}

            println!("{}", input);
		}
	}

	Ok(())
}
