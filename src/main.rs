extern crate csv;

mod tests;
mod sudoku;
mod file_io;

use std::path::Path;
use file_io::board_from_file;

fn main() {

    let template = board_from_file(&Path::new(&String::from("/Users/jacob/Documents/cs064/puzzles/sudoku/test.sudoku"))).unwrap();

    println!("{:?}", template);

    let mut sudoku = sudoku::Sudoku::new(template);
    let solved = sudoku.solve(sudoku::Position::new(0, 0)).unwrap();

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
