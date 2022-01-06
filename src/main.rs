use anyhow::Result;
use clap::Parser;
use log::*;
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
		debug!("Code to evaluate: `{}`", &code);

		let tree = parser
			.parse(&code, None)
			.expect("Couldn't parse the requested code...");
		let root_node = tree.root_node();
		debug!("{}", &root_node.to_sexp());

        let cursor = root_node.walk();
        walk(&cursor);
	}

	Ok(())
}
