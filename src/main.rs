use clap::Parser;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
	/// The pattern to look for
	pattern: String,
	/// The path to the file to read
	path: std::path::PathBuf,
}

fn main() -> io::Result<()> {

	let args = Cli::parse();

	let file = File::open(&args.path)?;
	let reader = BufReader::new(file);

	// let content = std::fs::read_to_string(&args.path).expect("could not read file");

	for line in reader.lines() {
		let line = line?;
		if line.contains(&args.pattern) {
			println!("{}",line);
		}
	}

	Ok(())

}
