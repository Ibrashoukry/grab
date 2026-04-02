use clap::Parser;
use anyhow::{Context, Result};
use std::io::{BufRead, BufReader};
use std::fs::File;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
	/// The pattern to look for
	pattern: String,
	/// The path to the file to read
	path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

	let args = Cli::parse();

	let file = File::open(&args.path)
		.with_context(|| format!("could not open file `{}`", args.path.display()))?;
	let reader = BufReader::new(file);

	for line in reader.lines() {
		let line = line?;
		if line.contains(&args.pattern) {
			println!("{}",line);
		}
	}

	Ok(())

}
