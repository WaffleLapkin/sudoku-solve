use bitvec::{array::BitArray, order::Lsb0};

use crate::board::Board;

pub trait Constraint {
    // TODO: cache results and instead ask what restrictions are added by placing something
    fn posibilities(
        &self,
        x: usize,
        y: usize,
        board: &Board,
        prev: BitArray<Lsb0, [u8; 2]>,
    ) -> BitArray<Lsb0, [u8; 2]>;
}
