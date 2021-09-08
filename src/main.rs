#![allow(dead_code)]
#![allow(unused)]
#![feature(iter_zip)]
#![feature(array_methods)]
use std::fmt::Display;
use std::fmt::{self, Write};
use std::fs::File;
use std::io::{self, Read};
use std::ops::{Deref, DerefMut};
use std::path::Path;

use bitvec::{
    array::BitArray,
    order::{Lsb0, Msb0},
};

pub mod board;
pub mod digit;
pub mod restriction;

use board::Board;
use digit::{Digit, MaybeDigit};

mod normal_sudoku_rules;
mod solution;
mod solve;
mod sudoku;

pub use self::{
    normal_sudoku_rules::NormalSudokuRules, solution::Solution, solve::solve, sudoku::Sudoku,
};

fn main() {
    let sudoku = Sudoku::read(Path::new("./sudoku")).expect("couldn't read");

    println!("Problem:\n{}", sudoku.board);

    match solve(&sudoku) {
        Err(()) => println!("No solution"),
        Ok(solution) => {
            println!("Solution:\n{}", solution.0)
        }
    };
}

/*

TODO: constraint

*/
