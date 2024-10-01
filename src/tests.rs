use std::path::PathBuf;
use crate::{generate, sudoku, SudokuT};

#[test]
fn easy() {
    let expected = vec![
        vec![4, 2, 9, 7, 6, 3, 8, 5, 1],
        vec![3, 7, 8, 4, 1, 5, 6, 2, 9],
        vec![5, 6, 1, 9, 2, 8, 7, 4, 3],
        vec![9, 8, 4, 6, 7, 2, 3, 1, 5],
        vec![6, 1, 3, 5, 4, 9, 2, 7, 8],
        vec![2, 5, 7, 8, 3, 1, 4, 9, 6],
        vec![8, 3, 2, 1, 5, 7, 9, 6, 4],
        vec![7, 4, 5, 3, 9, 6, 1, 8, 2],
        vec![1, 9, 6, 2, 8, 4, 5, 3, 7],
    ];

    let expected = map_option(expected);
    let actual = get_solved_vec("sudokus/sudoku_easy.txt");

    check_cells(expected, actual);
}

#[test]
fn hard() {
    let expected = vec![
        vec![5, 3, 1, 7, 2, 6, 9, 8, 4],
        vec![8, 2, 7, 4, 9, 5, 6, 1, 3],
        vec![6, 4, 9, 3, 1, 8, 2, 5, 7],
        vec![4, 9, 6, 5, 7, 1, 8, 3, 2],
        vec![7, 5, 3, 8, 4, 2, 1, 9, 6],
        vec![2, 1, 8, 9, 6, 3, 7, 4, 5],
        vec![9, 6, 2, 1, 5, 4, 3, 7, 8],
        vec![3, 7, 4, 2, 8, 9, 5, 6, 1],
        vec![1, 8, 5, 6, 3, 7, 4, 2, 9],
    ];

    let expected = map_option(expected);
    let actual = get_solved_vec("sudokus/sudoku_hard.txt");

    check_cells(expected, actual);
}

#[test]
fn empty() {
    let expected = vec![
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![4, 5, 6, 7, 8, 9, 1, 2, 3],
        vec![7, 8, 9, 1, 2, 3, 4, 5, 6],
        vec![2, 1, 4, 3, 6, 5, 8, 9, 7],
        vec![3, 6, 5, 8, 9, 7, 2, 1, 4],
        vec![8, 9, 7, 2, 1, 4, 3, 6, 5],
        vec![5, 3, 1, 6, 4, 2, 9, 7, 8],
        vec![6, 4, 2, 9, 7, 8, 5, 3, 1],
        vec![9, 7, 8, 5, 3, 1, 6, 4, 2],
    ];

    let expected = map_option(expected);
    let actual = get_solved_vec("sudokus/sudoku_empty.txt");

    check_cells(expected, actual);
}

fn check_cells(expected: SudokuT, actual: SudokuT) {
    for i in 0..9 {
        for j in 0..9 {
            assert_eq!(expected[i][j], actual[i][j], "Cell[{i}][{j}], \
            expected: {:?}, got: {:?}", expected[i][j], actual[i][j])
        }
    }
}

fn get_solved_vec(path: &'static str) -> SudokuT {
    let sudoku = generate::read_sudoku_file(
        PathBuf::from(path)
    ).unwrap();
    let mut sudoku = sudoku::Sudoku::from(sudoku);
    sudoku.solve();
    sudoku.into_inner()
}

fn map_option(raw: Vec<Vec<u8>>) -> SudokuT {
    let mut tmp = Vec::with_capacity(9);
    for row in raw {
        let mut inner_tmp = Vec::with_capacity(9);
        for cell in row {
            inner_tmp.push(Some(cell));
        }
        tmp.push(inner_tmp);
    }
    tmp
}