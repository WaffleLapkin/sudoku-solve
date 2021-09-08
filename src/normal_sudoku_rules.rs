use bitvec::{array::BitArray, order::Lsb0};

use crate::{board::Board, restriction::Restriction, MaybeDigit::Definitely};

pub struct NormalSudokuRules;

impl Restriction for NormalSudokuRules {
    fn posibilities(
        &self,
        x: usize,
        y: usize,
        board: &Board,
        mut prev: BitArray<Lsb0, [u8; 2]>,
    ) -> BitArray<Lsb0, [u8; 2]> {
        for &i in board
            .row(x)
            .into_iter()
            .chain(board.column(y))
            .chain(board.box_of((x, y)))
        {
            if let Definitely(i) = i {
                prev.set(i as _, false);
            }
        }

        prev
    }
}
