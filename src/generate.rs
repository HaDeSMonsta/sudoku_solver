use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;
use regex::Regex;
use crate::Sudoku;

pub fn read_sudoku_cmd() -> io::Result<Sudoku> {
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

pub fn read_sudoku_file(path: PathBuf) -> io::Result<Sudoku> {
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

pub fn parse_line(line: String) -> Vec<Option<u8>> {
	let re = Regex::new(r"[1-9-]").unwrap();

	let parts = line.trim().split(" ").collect::<Vec<_>>();

	assert_eq!(parts.len(), 9, "A valid line consists of 9 cells");

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
