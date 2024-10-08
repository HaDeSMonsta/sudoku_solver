default:
	@just --list

build:
	cargo build
	cargo build --release

skiena: build
	target/release/sudoku_solver --file sudokus/Skiena.txt --time

easy: build
	target/release/sudoku_solver --file sudokus/sudoku_easy.txt --time

medium: build
	target/release/sudoku_solver --file sudokus/sudoku_medium.txt --time

hard: build
	target/release/sudoku_solver --file sudokus/sudoku_hard.txt --time

# Might as well make it a cargo wrapper

test: build
	cargo test

clean:
	cargo clean

