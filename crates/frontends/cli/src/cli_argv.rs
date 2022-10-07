use std::path::PathBuf;

use clap::{self, Parser};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
	pub filepath: Option<PathBuf>,

	#[clap(short, long)]
	pub eval: Option<String>,

	#[clap(short, long)]
	pub check_only: bool,
}
