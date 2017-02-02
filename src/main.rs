extern crate csv;
extern crate clap;

mod tests;
mod sudoku;
mod file_io;

use clap::{App, Arg};
use std::path::Path;
use file_io::board_from_file;

fn main() {

    let matches = App::new("MyApp")
        .arg(
            Arg::with_name("input")
                .help("the input file to solve")
                .index(1)
                .required(true))
        .get_matches();

    if let Some(ref in_file) = matches.value_of("input") {
        
        let template = board_from_file(Path::new(&in_file)).unwrap();
        // let template = board_from_file(&Path::new(&String::from("/Users/jacob/Documents/cs064/puzzles/sudoku/test.sudoku"))).unwrap();

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
}
