use crate::SudokuT;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufWriter, Write};

#[derive(Clone)]
pub struct Sudoku {
    rows: SudokuT,
}

impl Sudoku {
    pub fn solve(&mut self) {
        if !self.solve_intern() { panic!("Sudoku is unsolvable"); }
    }

    fn solve_intern(&mut self) -> bool {
        if self.is_solved() { return true; }

        // find first option
        let mut x = 9;
        let mut y = 9;
        'outer: for row in 0..9 {
            for column in 0..9 {
                if self.rows
                       .get(row)
                       .unwrap()
                       .get(column)
                       .unwrap()
                       .is_none() {
                    x = row;
                    y = column;
                    break 'outer;
                }
            }
        }

        if x == 9 || y == 9 { return false; }

        // Try every possible value
        for num in 1..=9 {
            let cell = self.rows
                           .get_mut(x)
                           .unwrap()
                           .get_mut(y)
                           .unwrap();
            *cell = Some(num);
            if !self.is_valid() {
                // Need to shadow cell because we are calling a mut function before
                let cell = self.rows
                               .get_mut(x)
                               .unwrap()
                               .get_mut(y)
                               .unwrap();
                *cell = None;
                continue;
            }
            if self.solve_intern() { return true; }
            // Need to shadow cell because we are calling a mut function before
            let cell = self.rows
                           .get_mut(x)
                           .unwrap()
                           .get_mut(y)
                           .unwrap();
            *cell = None;
        }

        false
    }

    pub fn is_valid(&self) -> bool {
        // rc = row/column
        for rc in 0..9 {
            if !self.is_valid_row(rc) || !self.is_valid_column(rc) { return false; }
        }
        for row in 0..3 {
            for column in 0..3 {
                if !self.is_valid_box(row * 3, column * 3) { return false; }
            }
        }

        true
    }

    fn is_valid_row(&self, row: usize) -> bool {
        let row = self.rows.get(row).unwrap();
        let mut seen = HashSet::new();

        for cell in row {
            if let Some(num) = cell {
                if !seen.insert(*num) { return false; } // A number exists more than once
            }
        }

        true
    }

    fn is_valid_column(&self, column: usize) -> bool {
        let mut seen = HashSet::new();
        for row in 0..9 {
            if let Some(num) = self.rows
                                   .get(row)
                                   .unwrap()
                                   .get(column)
                                   .unwrap() {
                if !seen.insert(*num) { return false; }
            }
        }

        true
    }

    /// Pass in the top left corner of the 3x3 box
    fn is_valid_box(&self, row: usize, column: usize) -> bool {
        let mut seen = HashSet::new();

        for row in row..(row + 3) {
            for column in column..(column + 3) {
                if let Some(num) = self.rows
                                       .get(row)
                                       .unwrap()
                                       .get(column)
                                       .unwrap() {
                    if !seen.insert(*num) { return false; }
                }
            }
        }

        true
    }

    pub fn is_solved(&self) -> bool {
        for row in &self.rows {
            for cell in row {
                if cell.is_none() { return false; }
            }
        }
        self.is_valid()
    }

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

            println!();
            if sudoku_idx == sudoku.len() - 1 { break; }
            if (sudoku_idx + 1) % 3 == 0 { println!("{}", "-".repeat(34)); }
        }
    }

    /// Consume the Sudoku and write the content in a file
    /// The format will be the same as we expect as input
    pub fn dump_raw(self, f_name: &str) -> io::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(f_name)
            .expect(&format!("Unable to open {f_name}"));

        let mut writer = BufWriter::new(file);
        let sudoku = self.rows;

        for row in sudoku {
            for row_idx in 0..row.len() {
                let num = row.get(row_idx).unwrap().unwrap_or(0);
                if num == 0 {
                    write!(writer, "-")?;
                } else {
                    write!(writer, "{num}")?;
                }
                if row_idx != row.len() - 1 { write!(writer, " ")?; }
            }
            writeln!(writer)?;
        }

        Ok(())
    }

    /// For test extraction only
    #[cfg(test)]
    pub fn into_inner(self) -> SudokuT {
        self.rows
    }
}

impl From<SudokuT> for Sudoku {
    fn from(sudoku: SudokuT) -> Self {
        Self {
            rows: sudoku
        }
    }
}
