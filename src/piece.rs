use std::convert::TryFrom;
use std::fmt;
use std::option::Option;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ColoredPiece {
    Empty,
    P(Piece, Side),
}

impl fmt::Display for ColoredPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.symbol() {
            None => write!(f, ""),
            Some(s) => write!(f, "{}", s),
        }
    }
}

impl FromStr for ColoredPiece {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err("Line is too long")?
        }
        ColoredPiece::try_from(s.chars().next().unwrap())
    }
}

impl TryFrom<char> for ColoredPiece {
    type Error = &'static str;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'k' => Ok(named::k),
            'q' => Ok(named::q),
            'r' => Ok(named::r),
            'n' => Ok(named::n),
            'b' => Ok(named::b),
            'p' => Ok(named::p),
            'K' => Ok(named::K),
            'Q' => Ok(named::Q),
            'R' => Ok(named::R),
            'N' => Ok(named::N),
            'B' => Ok(named::B),
            'P' => Ok(named::P),
            _ => Err("Cannot convert char to ColoredPiece"),
        }
    }
}

impl ColoredPiece {
    pub fn symbol(&self) -> Option<char> {
        match *self {
            ColoredPiece::Empty => None,
            named::K => Some('K'),
            named::Q => Some('Q'),
            named::R => Some('R'),
            named::B => Some('B'),
            named::N => Some('N'),
            named::P => Some('P'),
            named::k => Some('k'),
            named::q => Some('q'),
            named::r => Some('r'),
            named::b => Some('b'),
            named::n => Some('n'),
            named::p => Some('p'),
        }
    }

    pub fn has_color(&self, s: Side) -> bool {
        match self {
            ColoredPiece::P(_, c) => *c == s,
            _ => false,
        }
    }
}

pub mod named {
    use super::*;

    pub const K: ColoredPiece = ColoredPiece::P(Piece::King, Side::White);
    pub const Q: ColoredPiece = ColoredPiece::P(Piece::Queen, Side::White);
    pub const R: ColoredPiece = ColoredPiece::P(Piece::Rook, Side::White);
    pub const B: ColoredPiece = ColoredPiece::P(Piece::Bishop, Side::White);
    pub const N: ColoredPiece = ColoredPiece::P(Piece::Knight, Side::White);
    pub const P: ColoredPiece = ColoredPiece::P(Piece::Pawn, Side::White);

    #[allow(non_upper_case_globals)]
    pub const k: ColoredPiece = ColoredPiece::P(Piece::King, Side::Black);
    #[allow(non_upper_case_globals)]
    pub const q: ColoredPiece = ColoredPiece::P(Piece::Queen, Side::Black);
    #[allow(non_upper_case_globals)]
    pub const r: ColoredPiece = ColoredPiece::P(Piece::Rook, Side::Black);
    #[allow(non_upper_case_globals)]
    pub const b: ColoredPiece = ColoredPiece::P(Piece::Bishop, Side::Black);
    #[allow(non_upper_case_globals)]
    pub const n: ColoredPiece = ColoredPiece::P(Piece::Knight, Side::Black);
    #[allow(non_upper_case_globals)]
    pub const p: ColoredPiece = ColoredPiece::P(Piece::Pawn, Side::Black);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Side {
    White,
    Black,
}

impl Side {
    pub fn symbol(&self) -> char {
        match *self {
            Side::White => 'w',
            Side::Black => 'b',
        }
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl FromStr for Side {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Side::White),
            "b" => Ok(Side::Black),
            _ => Err("Invalid color code"),
        }
    }
}
