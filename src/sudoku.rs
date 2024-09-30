use std::collections::HashSet;
use crate::SudokuT;

pub struct Sudoku {
    rows: SudokuT,
}

impl Sudoku {
    pub fn print(&self) {
        let sudoku = &self.rows;
        for sudoku_idx in 0..sudoku.len() {
            let row = sudoku.get(sudoku_idx).unwrap();

            for row_idx in 0..row.len() {
                let value = row.get(row_idx).unwrap().unwrap_or(0);
                if value == 0 { print!("-"); } else { print!("{value}"); }

                if row_idx == row.len() - 1 { break; }
                if (row_idx + 1) % 3 == 0 { print!(" ┃ "); } else { print!(" | "); }
            }

            if sudoku_idx == sudoku.len() - 1 { break; }
            println!();
            if (sudoku_idx + 1) % 3 == 0 { println!("{}", "-".repeat(34)); }
        }
    }

    pub fn solve(&mut self) {
        todo!()
    }

    pub fn valid(&self) -> bool {
        todo!()
    }

    fn validate_row(&self, row: u8) -> bool {
        let row = self.rows.get(row as usize).unwrap();
        let mut seen = HashSet::new();

        for cell in row {
            if let Some(num) = cell {
                if !seen.insert(*num) { return false; } // A number exists more than once
            }
        }

        true
    }

    fn validate_column(&self, column: u8) -> bool {
        let mut seen = HashSet::new();
        for row in 0usize..9usize {
            if let Some(num) = self.rows
                                   .get(row)
                                   .unwrap()
                                   .get(column as usize)
                                   .unwrap() {
                if !seen.insert(*num) { return false; }
            }
        }

        true
    }

    /// Pass in the top left corner of the 3x3 box
    fn validate_box(&self, row: u8, column: u8) -> bool {
        todo!()
    }

    pub fn solved(&self) -> bool {
        todo!()
    }
}

impl From<SudokuT> for Sudoku {
    fn from(sudoku: SudokuT) -> Self {
        Self {
            rows: sudoku
        }
    }
}
