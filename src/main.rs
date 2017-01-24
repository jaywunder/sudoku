/*
    Initialize 2D array with 81 empty grids (nx = 9, ny = 9)
    Fill in some empty grid with the known values
    Make an original copy of the array
    Start from top left grid (nx = 0, ny = 0), check if grid is empty
    if (grid is empty) {
        assign the empty grid with values (i)
        if (no numbers exists in same rows & same columns same as (i) & 3x3 square (i) is currently in)
            fill in the number
        if (numbers exists in same rows | same columns same as (i) | 3x3 square (i) is currently in)
            discard (i) and repick other values (i++)
    }
    else {
        while (nx < 9) {
            Proceed to next row grid(nx++, ny)
            if (nx equals 9) {
                reset nx = 1
                proceed to next column grid(nx,ny++)
                if (ny equals 9) {
                    print solution
                }
            }
        }
    }
*/

use std::io::{Error, ErrorKind};


type Board = [[Option<u8>; 9]; 9];

#[derive(Clone, Debug)]
struct Position {
    row: usize,
    col: usize,
}
impl Position {
    pub fn new(col: usize, row: usize) -> Position {
        Position {
            row: row,
            col: col,
        }
    }
}

struct Sudoku {
    board: Board,
}

impl Sudoku {

    pub fn new(template: Board) -> Sudoku {
        Sudoku {
            board: template,
        }
    }

    pub fn solve(&mut self, pos: Position) -> Result<Board, u8> {
        println!("{:?}", self.board);

        if let Some(_cell) = self.board[pos.row][pos.col] {

            if pos.col + 1 < self.board.len() {
                println!("increment column");
                return self.solve(Position::new(pos.col + 1, pos.row));

            } else if pos.row + 1 < self.board.len() {
                println!("increment row");
                return self.solve(Position::new(0, pos.row + 1));

            } else {
                println!("board is done");
                return Ok(self.board)
            }

            return Err(10);

        } else {
            for guess in 1..10 {

                println!("");
                println!(
                    "{:?} {:?} {:?}",
                    guess,
                    check_position(&self.board, &pos, guess),
                    pos.clone()
                );

                if check_position(&self.board, &pos, guess) {
                    self.board[pos.row][pos.col] = Some(guess);

                    let pos2 = pos.clone();
                    match self.solve(pos.clone()) {
                        Ok(board) => return Ok(board),
                        Err(_) => {
                            self.board[pos2.row][pos2.col] = None;
                        }
                    }
                }
            }

            return Err(10)
        }
    }

}

fn check_position(board: &Board, pos: &Position, guess: u8) -> bool {
    check_row(&board, pos.row, guess) &&
    check_col(&board, pos.col, guess) &&
    check_area(&board, &pos, guess)
}

fn check_row(board: &Board, row: usize, guess: u8) -> bool {
    !board[row].iter().any(|&x| x == Some(guess))
}

fn check_col(board: &Board, col: usize, guess: u8) -> bool {

    !board.iter().any(|&row| row[col] == Some(guess))

}

fn check_area(board: &Board, pos: &Position, guess: u8) -> bool {

    let start_row: usize = pos.row / 3 * 3;
    let start_col: usize = pos.col / 3 * 3;

    for row in start_row..start_row + 3 {
        for col in start_col..start_col + 3 {
            if board[row][col] == Some(guess){
                return false
            }
        }
    }

    true
}

fn main() {

    // let template: Board = [
    //     [None, None, None, None, None, None, None, None, None],
    //     [None, None, None, None, None, None, None, None, None],
    //     [None, None, None, None, None, None, None, None, None],
    //     [None, None, None, None, None, None, None, None, None],
    //     [None, None, None, None, None, None, None, None, None],
    //     [None, None, None, None, None, None, None, None, None],
    //     [None, None, None, None, None, None, None, None, None],
    //     [None, None, None, None, None, None, None, None, None],
    //     [None, None, None, None, None, None, None, None, None],
    // ];
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
    // let template: Board = [
    //     [None, Some(2), None, None, Some(6), Some(5), None, Some(8), None],
    //     [Some(9), None, Some(6), None, None, Some(7), Some(5), None, None],
    //     [Some(8), None, None, Some(2), None, None, Some(4), Some(6), None],
    //     [Some(2), None, None, None, None, None, Some(1), Some(3), Some(4)],
    //     [None, None, None, Some(7), Some(4), Some(6), None, None, None],
    //     [Some(5), Some(8), Some(4), None, None, None, None, None, Some(6)],
    //     [None, Some(1), Some(9), None, None, Some(4), None, None, Some(3)],
    //     [None, None, Some(2), Some(9), None, None, Some(6), None, Some(8)],
    //     [None, Some(5), None, Some(6), Some(2), None, None, Some(9), None],
    // ];

    let mut sudoku = Sudoku::new(template);
    let solved = sudoku.solve(Position::new(0, 0)).unwrap();

    println!("");
    for row in solved.iter() {
        for col in row.iter() {
            if let &Some(digit) = col {
                print!("{:<2}", digit);
            } else {
                print!("  ");
            }
        }
        println!("");
    }
}
