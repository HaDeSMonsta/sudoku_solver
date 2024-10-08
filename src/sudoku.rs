use crate::SudokuT;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufWriter, Write};

#[derive(Clone)]
pub struct Sudoku {
    rows: [[Option<u8>; 9]; 9],
}

impl Sudoku {
    pub fn solve(&mut self) {
        if !self.solve_intern() { panic!("Sudoku is unsolvable"); }
    }

    fn solve_intern(&mut self) -> bool {
        if self.is_solved() { return true; }

        // find first option
        let mut row = 9;
        let mut column = 9;
        'outer: for i in 0..9 {
            for j in 0..9 {
                if self.rows[i][j].is_none() {
                    row = i;
                    column = j;
                    break 'outer;
                }
            }
        }

        if row == 9 || column == 9 { return false; }

        // Try every possible value
        for num in 1..=9 {
            let cell = &mut self.rows[row][column];
            *cell = Some(num);
            if !self.is_valid(row, column) {
                // Need to shadow cell because we are calling a mut function before
                let cell = &mut self.rows[row][column];
                *cell = None;
                continue;
            }
            if self.solve_intern() { return true; }
            // Need to shadow cell because we are calling a mut function before
            let cell = &mut self.rows[row][column];
            *cell = None;
        }

        false
    }

    pub fn is_valid_full(&self) -> bool {
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

    fn is_valid(&self, row: usize, column: usize) -> bool {
        self.is_valid_row(row) &&
            self.is_valid_column(column) &&
            self.is_valid_box(
                3 * (row / 3),
                3 * (column / 3),
            )
    }

    fn is_valid_row(&self, row: usize) -> bool {
        let row = self.rows[row];
        let mut seen = HashSet::new();

        for cell in row {
            if let Some(num) = cell {
                if !seen.insert(num) { return false; } // A number exists more than once
            }
        }

        true
    }

    fn is_valid_column(&self, column: usize) -> bool {
        let mut seen = HashSet::new();
        for row in 0..9 {
            if let Some(num) = self.rows[row][column] {
                if !seen.insert(num) { return false; }
            }
        }

        true
    }

    /// Pass in the top left corner of the 3x3 box
    fn is_valid_box(&self, row: usize, column: usize) -> bool {
        let mut seen = HashSet::new();

        for row in row..(row + 3) {
            for column in column..(column + 3) {
                if let Some(num) = self.rows[row][column] {
                    if !seen.insert(num) { return false; }
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
        self.is_valid_full()
    }

    pub fn print(&self) {
        let sudoku = &self.rows;
        for sudoku_idx in 0..sudoku.len() {
            let row = sudoku[sudoku_idx];

            for row_idx in 0..row.len() {
                let cell = row[row_idx];
                if let Some(num) = cell {
                    print!("{num}");
                } else { print!("-") }

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
            for cell_idx in 0..row.len() {
                let cell = row[cell_idx];
                if let Some(num) = cell {
                    write!(writer, "{num}")?;
                } else { write!(writer, "-")?; }

                if cell_idx != row.len() - 1 { write!(writer, " ")?; }
            }
            writeln!(writer)?;
        }

        Ok(())
    }

    /// For test extraction only
    #[cfg(test)]
    pub fn into_inner(self) -> [[Option<u8>; 9]; 9] {
        self.rows
    }
}

impl From<SudokuT> for Sudoku {
    fn from(sudoku: SudokuT) -> Self {
        let mut rows = [[None; 9]; 9];

        let mut row_c = 0;
        for row in sudoku {
            let mut col_c = 0;
            for cell in row {
                rows[row_c][col_c] = cell;
                col_c += 1;
            }
            row_c += 1;
        }
        Self {
            rows
        }
    }
}
