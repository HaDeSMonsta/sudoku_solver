mod generate;
mod sudoku;

use clap::Parser;
use std::path::PathBuf;

use generate::*;
use crate::sudoku::Sudoku;

pub type SudokuT = Vec<Vec<Option<u8>>>;

/// Sudoku solver for the standard 9x9 Sudoku
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file with the Sudoku
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
    let mut sudoku = Sudoku::from(sudoku);
    println!("Read Sudoku:");
    sudoku.print();
    println!("\nIs valid: {}\nIs solved: {}\n", sudoku.is_valid(), sudoku.is_solved());
    sudoku.solve();
    println!("Solved Sudoku:");
    sudoku.print();
}


