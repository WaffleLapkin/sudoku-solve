use std::fmt::{self, Display};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MaybeDigit {
    Hole,
    Definitely(Digit),
}

impl Digit {
    pub fn new(x: u8) -> Self {
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
            _ => panic!("Expected number in range [1, 9], found {}", x),
        }
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&(*self as u8), f)
    }
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
