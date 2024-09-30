use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use clap::Parser;
use regex::Regex;

type Sudoku = Vec<Vec<Option<u8>>>;

/// Sudoku solver
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Input file with the sudoku
	#[arg(short, long)]
	file: Option<PathBuf>,
}

fn main() {
	let args = Args::parse();
	let sudoku;
	if let Some(path) = args.file {
		sudoku = read_sudoku_file(path);
	} else {
		sudoku = read_sudoku_cmd();
	};

	let sudoku = sudoku.expect("Unable to read input");
	print_sudoku(&sudoku);
}

fn print_sudoku(sudoku: &Sudoku) {
	for sudoku_idx in 0..sudoku.len() {
		let row = sudoku.get(sudoku_idx).unwrap();

		for row_idx in 0..row.len() {
			let value = row.get(row_idx).unwrap().unwrap_or(0);
			if value == 0 { print!("-"); } else { print!("{value}"); }

			if row_idx == row.len() - 1 { break; }
			if (row_idx + 1) % 3 == 0 { print!(" ┃ "); }
			else { print!(" | "); }
		}

		if sudoku_idx == sudoku.len() - 1 { break; }
		println!();
		if (sudoku_idx + 1) % 3 == 0 { println!("{}", "-".repeat(34)); }
	}
}

fn read_sudoku_cmd() -> io::Result<Sudoku> {
	let mut sudoku = Vec::new();
	println!("Please input the sudoku now line by line, allowed characters: \
	[1-9-], separate by space");
	for _ in 0..9 {
		let mut input = String::new();
		io::stdin().read_line(&mut input)?;
		let row = parse_line(input);
		sudoku.push(row);
	}

	Ok(sudoku)
}

fn read_sudoku_file(path: PathBuf) -> io::Result<Sudoku> {
	println!("Reading sudoku from file {path:?}");

	let file = OpenOptions::new()
		.read(true)
		.open(path)?;
	let reader = BufReader::new(file);

	let mut sudoku = Vec::new();
	for line in reader.lines() {
		sudoku.push(parse_line(line?));
	}
	Ok(sudoku)
}

fn parse_line(line: String) -> Vec<Option<u8>> {
	let re = Regex::new(r"[1-9-]").unwrap();

	let parts = line.trim().split(" ").collect::<Vec<_>>();

	assert_eq!(parts.len(), 9, "A line consists of 9 cells");

	let mut row = Vec::new();
	let mut seen = HashSet::new();

	for part in parts {
		assert!(re.is_match(part), "A cell can only be 1-9 or a '-'");

		if part == "-" {
			row.push(None);
			continue;
		}

		let num = part.parse().unwrap();

		assert!(seen.insert(num), "A row cannot contain duplicates");

		row.push(Some(num));
	}

	row
}