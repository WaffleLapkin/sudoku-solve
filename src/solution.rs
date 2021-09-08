use crate::{board::Board, digit::Digit};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Solution(pub Board<Digit>);
