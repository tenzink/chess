#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ColoredPiece {
    Empty,
    P(Piece, Side),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Piece {
    Empty,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Side {
    Empty,
    White,
    Black,
}
