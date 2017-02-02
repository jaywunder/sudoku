#![allow(unused_imports, dead_code, unused_variables)]
use super::sudoku::Board;

use std::convert::From;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use std::error;
// use std::error::*;

use csv;
use csv::{Error, StringRecords};

pub fn board_from_file(path: &Path) -> Result<Board, &error::Error> {

    let mut board: [[Option<u8>; 9]; 9] = [[None; 9]; 9];

    let mut board_reader = csv::Reader::from_file(path)
                                    .unwrap()
                                    .has_headers(false)
                                    .delimiter(b' ')
                                    .flexible(true);

    for (i, row) in board_reader.records().enumerate() {
        let row = row.unwrap();

        let row_vec: Vec<Option<u8>> = row.iter().map(|value| {
            match value.as_bytes()[0] {
                b'.' => None,
                b'0' => Some(0), b'1' => Some(1),
                b'2' => Some(2), b'3' => Some(3),
                b'4' => Some(4), b'5' => Some(5),
                b'6' => Some(6), b'7' => Some(7),
                b'8' => Some(8), b'9' => Some(9),
                _ => None
            }
        }).collect();

        for (j, value) in row_vec.iter().take(9).enumerate() {
            board[i][j] = match value {
                &Some(46) => None,
                &Some(n) => Some(n),
                &None => None
            }
        }
    }

    Ok(board)
}

pub fn board_to_file<'a>(path: &Path, board: &Board) -> Result<(), &'a error::Error> {
    Ok(())
}
