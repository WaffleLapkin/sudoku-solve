use std::io::Read;
use std::{fs::File, io, path::Path};

use crate::Digit::*;
use crate::MaybeDigit::Definitely;
use crate::{board::Board, normal_sudoku_rules::NormalSudokuRules};

pub struct Sudoku<R> {
    pub board: Board,
    pub restrictions: R,
}

impl Sudoku<NormalSudokuRules> {
    pub fn read(path: &Path) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(Self::parse(&contents))
    }

    pub fn parse(contents: &str) -> Self {
        let mut this = Self {
            board: <_>::default(),
            restrictions: NormalSudokuRules,
        };

        assert_eq!(contents.splitn(9, '\n').count(), 9);
        for (x, line) in contents.splitn(9, '\n').enumerate() {
            for (y, digit) in line.split_whitespace().enumerate() {
                assert_eq!(line.split_whitespace().count(), 9);

                let digit = match digit {
                    "1" => One,
                    "2" => Two,
                    "3" => Three,
                    "4" => Four,
                    "5" => Five,
                    "6" => Six,
                    "7" => Seven,
                    "8" => Eight,
                    "9" => Nine,
                    _ => continue,
                };

                this.board[(x, y)] = Definitely(digit);
            }
        }

        this
    }
}
