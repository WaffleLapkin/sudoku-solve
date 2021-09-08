use std::{
    array,
    fmt::{self, Display, Write},
    iter, mem,
    ops::{Index, IndexMut},
    slice,
};

use crate::MaybeDigit;

/// Sudoku board.
///
/// 9 by 9 grid, row are numbered top to bottom, colums are numbered left to right. In the inner array the first index is rows, the second is collums, ie you should index it as `[x][y]` (`[row][column]`)
///
/// ```text
/// + -----------------------------------------> y
/// |  0,  1,  2,  3,  4,  5,  6,  7,  8,  9,
/// | 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
/// | 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
/// | 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
/// | 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
/// | 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
/// | 60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
/// | 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
/// | 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
/// |
/// v
/// x
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Board<T = MaybeDigit>([[T; 9]; 9]);

impl<T> Board<T> {
    pub fn new(v: [[T; 9]; 9]) -> Self {
        Self(v)
    }

    pub fn cell(&self, idx: usize) -> &T {
        &self.0[idx / 9][idx % 9]
    }

    pub fn cell_mut(&self, idx: usize) -> &T {
        &self.0[idx / 9][idx % 9]
    }

    pub fn cells(&self) -> impl Iterator<Item = &T> {
        self.0.iter().flatten()
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.iter_mut().flatten()
    }

    pub fn row(&self, idx: usize) -> Group<T> {
        Group(self.0[idx].each_ref())
    }

    pub fn row_mut(&mut self, idx: usize) -> GroupMut<T> {
        GroupMut(self.0[idx].each_mut())
    }

    pub fn rows(&self) -> impl Iterator<Item = Group<T>> {
        self.0.iter().map(|row| Group(row.each_ref()))
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = GroupMut<T>> {
        self.0.iter_mut().map(|row| GroupMut(row.each_mut()))
    }

    pub fn column(&self, idx: usize) -> Group<T> {
        Group(self.0.each_ref().map(|row| &row[idx]))
    }

    pub fn column_mut(&mut self, idx: usize) -> GroupMut<T> {
        GroupMut(self.0.each_mut().map(|row| &mut row[idx]))
    }

    pub fn columns(&self) -> impl Iterator<Item = Group<T>> {
        (0..9).map(move |y| Group([0, 1, 2, 3, 4, 5, 6, 7, 8].map(|x| &self.0[x][y])))
    }

    pub fn columns_mut(&mut self) -> impl Iterator<Item = GroupMut<T>> {
        fn take_first<'b, T>(slice: &mut &'b mut [T]) -> &'b mut T {
            match mem::take(slice) {
                [first, rest @ ..] => {
                    *slice = rest;
                    first
                }
                [] => unreachable!(),
            }
        }

        let mut array: [&mut [_]; 9] = self.0.each_mut().map(|row| &mut row[..]);

        (0..9).map(move |_| GroupMut(array.each_mut().map(take_first)))
    }

    #[rustfmt::skip]
    pub fn box_of(&self, (x, y): (usize, usize)) -> Group<T> {
        let x = x - (x % 3);
        let y = y - (y % 3);

        Group([
            &self.0[x][y],     &self.0[x][y + 1],     &self.0[x][y + 2],
            &self.0[x + 1][y], &self.0[x + 1][y + 1], &self.0[x + 1][y + 2],
            &self.0[x + 2][y], &self.0[x + 2][y + 1], &self.0[x + 2][y + 2],
        ])
    }

    pub fn map<F, U>(self, mut f: F) -> Board<U>
    where
        F: FnMut(T) -> U,
    {
        Board(self.0.map(|row| row.map(&mut f)))
    }
}

impl<T: Display> Display for Board<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (x, row) in self.rows().enumerate() {
            for (y, digit) in row.into_iter().enumerate() {
                Display::fmt(digit, f)?;

                if y != 8 {
                    f.write_char(' ')?;
                }
            }

            if x != 8 {
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}

impl<T> Index<(usize, usize)> for Board<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.0[x][y]
    }
}

impl<T> IndexMut<(usize, usize)> for Board<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.0[x][y]
    }
}

pub struct Group<'a, T: 'a>([&'a T; 9]);

impl<T> Index<usize> for Group<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &*self.0[index]
    }
}

impl<'a, T: 'a> IntoIterator for Group<'a, T> {
    type Item = &'a T;

    type IntoIter = array::IntoIter<&'a T, 9>;

    fn into_iter(self) -> Self::IntoIter {
        <_>::into_iter(self.0)
    }
}

pub struct GroupMut<'a, T: 'a>([&'a mut T; 9]);

impl<T> Index<usize> for GroupMut<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &*self.0[index]
    }
}

impl<T> IndexMut<usize> for GroupMut<'_, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut *self.0[index]
    }
}

impl<'a, T: 'a> IntoIterator for GroupMut<'a, T> {
    type Item = &'a mut T;

    type IntoIter = array::IntoIter<&'a mut T, 9>;

    fn into_iter(self) -> Self::IntoIter {
        <_>::into_iter(self.0)
    }
}
