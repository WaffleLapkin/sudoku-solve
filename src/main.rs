#![allow(dead_code)]
#![allow(unused)]
#![feature(iter_zip)]
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

// + -----------------------------------------> y
// |  0,  1,  2,  3,  4,  5,  6,  7,  8,  9,
// | 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
// | 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
// | 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
// | 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
// | 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
// | 60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
// | 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
// | 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
// |
// v
// x
//
// [x][y]
#[derive(Debug, Default, Clone)]
pub struct Board<T = MaybeDigit>([[T; 9]; 9]);

impl<T: Display> fmt::Display for Board<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (x, row) in self.0.iter().enumerate() {
            for (y, digit) in row.iter().enumerate() {
                Display::fmt(digit, f)?;

                if y != 8 {
                    f.write_char(' ')?;
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl<T> Deref for Board<T> {
    type Target = [[T; 9]; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Board<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Digit {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&(*self as u8), f)
    }
}

impl Digit {
    fn new(x: u8) -> Self {
        use Digit::*;

        match x {
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            6 => Six,
            7 => Seven,
            8 => Eight,
            9 => Nine,
            _ => todo!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MaybeDigit {
    Hole,
    Definitely(Digit),
}

impl Display for MaybeDigit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hole => f.write_str("_"),
            Self::Definitely(d) => Display::fmt(d, f),
        }
    }
}

impl Default for MaybeDigit {
    fn default() -> Self {
        Self::Hole
    }
}

struct Sudoku<R> {
    board: Board,
    restrictions: R,
}

impl Sudoku<NormalSudokuRules> {
    fn read(path: &Path) -> io::Result<Self> {
        use Digit::*;

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

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

                this.board[x][y] = Definitely(digit);
            }
        }

        Ok(this)
    }
}

struct SolveState<'a, R> {
    holes: BitArray<Lsb0, [u8; 11]>,
    board: Board,
    restrictions: &'a R,
}

impl<'a, R> SolveState<'a, R> {
    fn new(sudoku: &'a Sudoku<R>) -> Self {
        let mut holes = BitArray::default();

        for idx in sudoku
            .board
            .iter()
            .flatten()
            .enumerate()
            .flat_map(|(i, d)| match d {
                Definitely(_) => None,
                Hole => Some(i),
            })
        {
            holes.set(idx, true);
        }

        Self {
            holes,
            board: sudoku.board.clone(),
            restrictions: &sudoku.restrictions,
        }
    }
}

#[derive(Debug)]
pub struct Solution(pub Board<Digit>);

struct NormalSudokuRules;

impl Restriction for NormalSudokuRules {
    fn posibilities(
        &self,
        x: usize,
        y: usize,
        board: &Board,
        mut prev: BitArray<Lsb0, [u8; 2]>,
    ) -> BitArray<Lsb0, [u8; 2]> {
        let row = board[x].iter();
        let column = board.iter().map(|row| &row[y]);
        let box_ = {
            let x = x - (x % 3);
            let y = y - (y % 3);

            (x..x + 3).flat_map(move |x_| (y..y + 3).map(move |y_| &board[x_][y_]))
        };

        for &i in row.chain(column).chain(box_) {
            if let Definitely(i) = i {
                prev.set(i as _, false);
            }
        }

        prev
    }
}

trait Restriction {
    // TODO: cache results and instead ask what restrictions are added by placing something
    fn posibilities(
        &self,
        x: usize,
        y: usize,
        board: &Board,
        prev: BitArray<Lsb0, [u8; 2]>,
    ) -> BitArray<Lsb0, [u8; 2]>;
}

fn solve<R>(sudoku: &Sudoku<R>) -> Result<Solution, ()>
where
    R: Restriction,
{
    let mut state = SolveState::new(sudoku);

    solve_step(&mut state, &Recursion::zero())?;

    let solution = state.board.0.map(|row| {
        row.map(|x| match x {
            Definitely(n) => n,
            Hole => unreachable!(),
        })
    });

    Ok(Solution(Board(solution)))
}

use MaybeDigit::*;

struct Recursion(u32);

impl Recursion {
    const fn zero() -> Self {
        Self(0)
    }

    fn step(&self) -> Self {
        Self(self.0 + 1)
    }
}

fn solve_step<R>(state: &mut SolveState<R>, rec: &Recursion) -> Result<(), ()>
where
    R: Restriction,
{
    let (x, y, idx) = match state.holes.iter_ones().next() {
        Some(hole) => (hole / 9, hole % 9, hole),
        None => return Ok(()),
    };

    state.holes.set(idx, false);

    let mut default_posibilities = BitArray::zeroed();
    default_posibilities[1..=9].set_all(true);
    for digit in state
        .restrictions
        .posibilities(x, y, &state.board, default_posibilities)
        .iter_ones()
        .map(|p| p as u8)
        .map(Digit::new)
        .map(Definitely)
    {
        state.board[x][y] = digit;

        if let ok @ Ok(_) = solve_step(state, &rec.step()) {
            return ok;
        }
    }

    state.board[x][y] = Hole;
    state.holes.set(idx, true);

    Err(())
}

/*

TODO: constraint

Basic tests for regressions

Problem:
3 7 5 6 2 _ _ _ 4
4 9 2 3 _ 8 _ 5 6
_ _ _ _ 9 4 _ 7 _
7 4 8 1 5 _ 6 3 2
1 2 _ 8 6 _ _ 4 5
6 5 9 2 _ 3 7 _ _
2 _ 4 7 3 6 5 8 9
_ 8 6 4 1 5 3 2 _
_ 3 7 _ 8 2 _ 6 1

Solution:
3 7 5 6 2 1 8 9 4
4 9 2 3 7 8 1 5 6
8 6 1 5 9 4 2 7 3
7 4 8 1 5 9 6 3 2
1 2 3 8 6 7 9 4 5
6 5 9 2 4 3 7 1 8
2 1 4 7 3 6 5 8 9
9 8 6 4 1 5 3 2 7
5 3 7 9 8 2 4 6 1

Problem:
_ _ 1 _ _ _ 7 _ _
_ _ _ _ 1 _ 4 8 5
8 4 _ 6 _ _ _ 3 _
5 _ _ 1 9 _ _ _ _
_ _ 3 5 _ _ _ _ 6
_ _ _ _ _ _ 5 _ _
_ 5 9 3 _ _ 6 4 _
1 8 _ 7 2 _ _ _ 3
3 _ _ _ _ _ _ _ _

Solution:
2 3 1 4 5 8 7 6 9
6 9 7 2 1 3 4 8 5
8 4 5 6 7 9 2 3 1
5 6 8 1 9 7 3 2 4
9 7 3 5 4 2 8 1 6
4 1 2 8 3 6 5 9 7
7 5 9 3 8 1 6 4 2
1 8 6 7 2 4 9 5 3
3 2 4 9 6 5 1 7 8

// https://www.youtube.com/watch?v=Ui1hrp7rovw
Problem:
_ _ _ 1 _ 2 _ _ _
_ 6 _ _ _ _ _ 7 _
_ _ 8 _ _ _ 9 _ _
4 _ _ _ _ _ _ _ 3
_ 5 _ _ _ 7 _ _ _
2 _ _ _ 8 _ _ _ 1
_ _ 9 _ _ _ 8 _ 5
_ 7 _ _ _ _ _ 6 _
_ _ _ 3 _ 4 _ _ _

Solution:
9 3 4 1 7 2 6 5 8
5 6 1 9 4 8 3 7 2
7 2 8 6 3 5 9 1 4
4 1 7 2 6 9 5 8 3
8 5 3 4 1 7 2 9 6
2 9 6 5 8 3 7 4 1
1 4 9 7 2 6 8 3 5
3 7 2 8 5 1 4 6 9
6 8 5 3 9 4 1 2 7
*/
