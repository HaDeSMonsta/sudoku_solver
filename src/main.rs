mod generate;

use clap::Parser;
use std::path::PathBuf;

use generate::*;

pub type Sudoku = Vec<Vec<Option<u8>>>;

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

