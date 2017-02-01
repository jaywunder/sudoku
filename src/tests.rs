#![allow(unused_imports)]
use super::sudoku::{ Sudoku, Board, Position };

#[test]
fn test_base_case() {
    let template: Board = [
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
    ];

    let mut sudoku = Sudoku::new(template);
    let solved = sudoku.solve(Position::new(0, 0)).unwrap();
    let answer: Board = [
        [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9)],
        [Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3)],
        [Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)],
        [Some(2), Some(1), Some(4), Some(3), Some(6), Some(5), Some(8), Some(9), Some(7)],
        [Some(3), Some(6), Some(5), Some(8), Some(9), Some(7), Some(2), Some(1), Some(4)],
        [Some(8), Some(9), Some(7), Some(2), Some(1), Some(4), Some(3), Some(6), Some(5)],
        [Some(5), Some(3), Some(1), Some(6), Some(4), Some(2), Some(9), Some(7), Some(8)],
        [Some(6), Some(4), Some(2), Some(9), Some(7), Some(8), Some(5), Some(3), Some(1)],
        [Some(9), Some(7), Some(8), Some(5), Some(3), Some(1), Some(6), Some(4), Some(2)],
    ];

    assert_eq!(solved, answer)
}

#[test]
fn test_sudoku1() {
    let template: Board = [
        [None, Some(2), None, None, Some(6), Some(5), None, Some(8), None],
        [Some(9), None, Some(6), None, None, Some(7), Some(5), None, None],
        [Some(8), None, None, Some(2), None, None, Some(4), Some(6), None],
        [Some(2), None, None, None, None, None, Some(1), Some(3), Some(4)],
        [None, None, None, Some(7), Some(4), Some(6), None, None, None],
        [Some(5), Some(8), Some(4), None, None, None, None, None, Some(6)],
        [None, Some(1), Some(9), None, None, Some(4), None, None, Some(3)],
        [None, None, Some(2), Some(9), None, None, Some(6), None, Some(8)],
        [None, Some(5), None, Some(6), Some(2), None, None, Some(9), None],
    ];

    let mut sudoku = Sudoku::new(template);
    let solved = sudoku.solve(Position::new(0, 0)).unwrap();
    let answer: Board = [
        [Some(7), Some(2), Some(1), Some(4), Some(6), Some(5), Some(3), Some(8), Some(9)],
        [Some(9), Some(4), Some(6), Some(3), Some(8), Some(7), Some(5), Some(1), Some(2)],
        [Some(8), Some(3), Some(5), Some(2), Some(1), Some(9), Some(4), Some(6), Some(7)],
        [Some(2), Some(6), Some(7), Some(5), Some(9), Some(8), Some(1), Some(3), Some(4)],
        [Some(1), Some(9), Some(3), Some(7), Some(4), Some(6), Some(8), Some(2), Some(5)],
        [Some(5), Some(8), Some(4), Some(1), Some(3), Some(2), Some(9), Some(7), Some(6)],
        [Some(6), Some(1), Some(9), Some(8), Some(7), Some(4), Some(2), Some(5), Some(3)],
        [Some(3), Some(7), Some(2), Some(9), Some(5), Some(1), Some(6), Some(4), Some(8)],
        [Some(4), Some(5), Some(8), Some(6), Some(2), Some(3), Some(7), Some(9), Some(1)],
    ];

    assert_eq!(solved, answer)
}

#[test]
fn test_sudoku2() {
    let template: Board = [
        [None, Some(1), None, Some(5), None, None, Some(7), None, Some(6)],
        [Some(7), None, Some(4), Some(6), Some(2), None, None, None, None],
        [None, None, Some(3), None, None, Some(7), Some(5), Some(8), None],
        [Some(3), Some(4), None, Some(8), None, None, None, Some(7), None],
        [None, None, None, None, Some(7), None, None, None, None],
        [None, Some(2), None, None, None, Some(5), None, Some(6), Some(8)],
        [None, Some(9), Some(8), Some(4), None, None, Some(1), None, None],
        [None, None, None, None, Some(6), Some(9), Some(8), None, Some(2)],
        [Some(5), None, Some(2), None, None, Some(3), None, Some(4), None],
    ];

    let mut sudoku = Sudoku::new(template);
    let solved = sudoku.solve(Position::new(0, 0)).unwrap();
    let answer: Board = [
        [Some(8), Some(1), Some(9), Some(5), Some(3), Some(4), Some(7), Some(2), Some(6)],
        [Some(7), Some(5), Some(4), Some(6), Some(2), Some(8), Some(3), Some(9), Some(1)],
        [Some(2), Some(6), Some(3), Some(9), Some(1), Some(7), Some(5), Some(8), Some(4)],
        [Some(3), Some(4), Some(6), Some(8), Some(9), Some(1), Some(2), Some(7), Some(5)],
        [Some(9), Some(8), Some(5), Some(2), Some(7), Some(6), Some(4), Some(1), Some(3)],
        [Some(1), Some(2), Some(7), Some(3), Some(4), Some(5), Some(9), Some(6), Some(8)],
        [Some(6), Some(9), Some(8), Some(4), Some(5), Some(2), Some(1), Some(3), Some(7)],
        [Some(4), Some(3), Some(1), Some(7), Some(6), Some(9), Some(8), Some(5), Some(2)],
        [Some(5), Some(7), Some(2), Some(1), Some(8), Some(3), Some(6), Some(4), Some(9)],
    ];

    assert_eq!(solved, answer)
}
