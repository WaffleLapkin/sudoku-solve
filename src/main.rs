#![allow(dead_code)]
#![allow(unused)]
#![feature(iter_zip)]
use bitvec::{
    array::BitArray,
    order::{Lsb0, Msb0},
};

fn main() {
    println!("Hello, world!");
}

type Board<T = MaybeDigit> = [[T; 9]; 9];

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
enum MaybeDigit {
    Definitely(Digit),
    Hole,
}

struct Sudoku<R> {
    board: Board,
    restrictions: R,
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

pub struct Solution(pub Board<Digit>);

struct NormalSudokuRules;

impl Restriction for NormalSudokuRules {
    fn posibilities(&self, x: usize, y: usize, board: &Board) -> BitArray<Lsb0, [u8; 2]> {}
}

trait Restriction {
    fn posibilities(&self, x: usize, y: usize, board: &Board) -> BitArray<Lsb0, [u8; 2]>;
}

fn solve<R>(sudoku: &Sudoku<R>) -> Result<Solution, ()>
where
    R: Restriction,
{
    let mut state = SolveState::new(sudoku);

    solve_step(&mut state, &Recursion::zero())?;

    let solution = state.board.map(|row| {
        row.map(|x| match x {
            Definitely(n) => n,
            Hole => unreachable!(),
        })
    });

    Ok(Solution(solution))
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

    state.holes.set(idx, true);

    for digit in state
        .restrictions
        .posibilities(x, y, &state.board)
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
    state.holes.set(idx, false);

    Err(())
}
