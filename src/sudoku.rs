use crate::SudokuT;

pub struct Sudoku {
	rows: Vec<Vec<Option<u8>>>
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
				if (row_idx + 1) % 3 == 0 { print!(" ┃ "); }
				else { print!(" | "); }
			}

			if sudoku_idx == sudoku.len() - 1 { break; }
			println!();
			if (sudoku_idx + 1) % 3 == 0 { println!("{}", "-".repeat(34)); }
		}
	}
}

impl From<SudokuT> for Sudoku {
	fn from(sudoku: SudokuT) -> Self {
		Self {
			rows: sudoku
		}
	}
}
