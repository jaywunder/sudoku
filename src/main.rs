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

        if let Some(_cell) = self.board[pos.row][pos.col] {

            if pos.col + 1 < self.board.len() {

                self.solve(Position::new(pos.col + 1, pos.row))

            } else if pos.row + 1 < self.board.len() {

                self.solve(Position::new(0, pos.row + 1))

            } else {

                Ok(self.board)
                
            }

        } else {
            for guess in 1..10 {

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
