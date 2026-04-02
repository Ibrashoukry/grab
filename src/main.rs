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
	/// Enable search to be case insensitive
	#[arg(short = 'i', long = "case-insensitive")]
	case_insensitive: bool,
	/// Output line number of line that contains the pattern
	#[arg(short = 'n', long = "line-number")]
	line_number: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

	let args = Cli::parse();
	let file = File::open(&args.path)
		.with_context(|| format!("could not open file `{}`", args.path.display()))?;
	let reader = BufReader::new(file);

	// set pattern based on case sensitivity
	let pattern = if args.case_insensitive {
		args.pattern.to_lowercase()
	} else {
		args.pattern.clone()
	};

	for (n, line) in reader.lines().enumerate() {
		let line = line
			.with_context(|| format!("Error reading line `{}`", n))?;

		let compare = if args.case_insensitive {
			line.to_lowercase()
		} else {
			line.clone()
		};

		if compare.contains(&pattern) {
			// adds line number if flag is inputted
			if args.line_number {
				print!("{}: ", n + 1);
			}
			println!("{}",line);
		}
	}

	Ok(())

}
