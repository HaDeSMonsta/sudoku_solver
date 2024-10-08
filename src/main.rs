mod generate;
mod sudoku;
#[cfg(test)]
mod tests;

use clap::Parser;
use std::io;
use std::path::PathBuf;
use std::time::Instant;
use crate::sudoku::Sudoku;
use generate::*;

pub type SudokuT = Vec<Vec<Option<u8>>>;

/// Sudoku solver for the standard 9x9 Sudoku
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file with the Sudoku
    #[arg(short, long)]
    file: Option<PathBuf>,
    /// If you want to time how long solving takes
    #[arg(short, long)]
    time: bool,
}

fn main() {
    let args = Args::parse();
    let sudoku;
    let via_cmd;
    if let Some(path) = args.file {
        sudoku = read_sudoku_file(path);
        via_cmd = false;
    } else {
        sudoku = read_sudoku_cmd();
        via_cmd = true;
    };

    let sudoku = sudoku.expect("Unable to read input");
    let mut sudoku = Sudoku::from(sudoku);
    let cloned = if via_cmd { Some(sudoku.clone()) } else { None };
    println!("Read Sudoku:");
    sudoku.print();
    #[cfg(debug_assertions)]
    println!("\nIs valid: {}\nIs solved: {}\n", sudoku.is_valid(), sudoku.is_solved());
    #[cfg(not(debug_assertions))]
    println!();

    let start = Instant::now();

    sudoku.solve();

    let elapsed = start.elapsed();
    let time_str = if args.time {
        format!(" ({} ms)", elapsed.as_millis())
    } else {
        String::new()
    };

    println!("Solved Sudoku:{}", time_str);
    sudoku.print();

    if !via_cmd { return; }

    println!("Do you want to save the input sudoku in a file? [filename|blank]");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    if input.is_empty() {
        println!("Not saving, goodbye");
        return;
    }

    let sudoku = cloned.unwrap();
    sudoku.dump_raw(input).unwrap();
    println!("Saved to {input}, goodbye");
}

