mod generate;
mod sudoku;

use clap::Parser;
use std::path::PathBuf;

use generate::*;
use crate::sudoku::Sudoku;

pub type SudokuT = Vec<Vec<Option<u8>>>;

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
	let sudoku = Sudoku::from(sudoku);
	sudoku.print();
}


