#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ColoredPiece {
    Empty,
    P(Piece, Side),
}

impl ColoredPiece {
    pub fn has_color(&self, s: Side) -> bool {
        match self {
            ColoredPiece::P(_, c) => *c == s,
            _ => false,
        }
    }

    pub fn is_white(&self) -> bool {
        self.has_color(Side::White)
    }

    pub fn is_black(&self) -> bool {
        self.has_color(Side::Black)
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
