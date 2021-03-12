use crate::field::{named::*, Field, COUNT};
use crate::piece::Piece;
use crate::side::Side;

#[derive(Debug, PartialEq)]
pub struct Board {
    pub sides: [Side; COUNT],
    pub pieces: [Piece; COUNT],
    pub active: Side,
    pub can_castle: [bool; 4], // white-king, white-queen, black-king, black-queen
    pub en_passant: Option<Field>,
    pub halfmove_clock: u32,
    pub full_moves: u32,
}

impl Board {
    pub fn side(&self, f: Field) -> Side {
        self.sides[f.0]
    }

    pub fn piece(&self, f: Field) -> Piece {
        self.pieces[f.0]
    }

    pub fn new() -> Board {
        Board::from(&[], Side::White, [true, true, true, true], None, 0, 1)
    }

    pub fn from(
        list: &[(Field, Side, Piece)],
        active: Side,
        can_castle: [bool; 4],
        en_passant: Option<Field>,
        halfmove_clock: u32,
        full_moves: u32,
    ) -> Board {
        let mut sides = [Side::Empty; COUNT];
        let mut pieces = [Piece::Empty; COUNT];
        for (idx, side, piece) in list {
            sides[idx.0] = *side;
            pieces[idx.0] = *piece;
        }
        Board {
            sides,
            pieces,
            active,
            can_castle,
            en_passant,
            halfmove_clock,
            full_moves,
        }
    }

    pub fn initial() -> Board {
        use crate::piece::Piece::*;
        use crate::side::Side::*;
        const LIST: [(Field, Side, Piece); 32] = [
            (A1, White, Rook),
            (B1, White, Knight),
            (C1, White, Bishop),
            (D1, White, Queen),
            (E1, White, King),
            (F1, White, Bishop),
            (G1, White, Knight),
            (H1, White, Rook),
            (A2, White, Pawn),
            (B2, White, Pawn),
            (C2, White, Pawn),
            (D2, White, Pawn),
            (E2, White, Pawn),
            (F2, White, Pawn),
            (G2, White, Pawn),
            (H2, White, Pawn),
            (A7, Black, Pawn),
            (B7, Black, Pawn),
            (C7, Black, Pawn),
            (D7, Black, Pawn),
            (E7, Black, Pawn),
            (F7, Black, Pawn),
            (G7, Black, Pawn),
            (H7, Black, Pawn),
            (A8, Black, Rook),
            (B8, Black, Knight),
            (C8, Black, Bishop),
            (D8, Black, Queen),
            (E8, Black, King),
            (F8, Black, Bishop),
            (G8, Black, Knight),
            (H8, Black, Rook),
        ];
        Board::from(&LIST, Side::White, [true, true, true, true], None, 0, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{fields, row};

    #[test]
    fn new() {
        let b = Board::new();
        for i in fields() {
            assert_eq!(b.side(i), Side::Empty);
            assert_eq!(b.piece(i), Piece::Empty);
        }
    }

    #[test]
    fn initial() {
        let b = Board::initial();
        for r in 1..3 {
            for idx in row(r) {
                assert_eq!(b.side(idx), Side::White, "Expect white piece at {}", idx);
            }
        }
        for r in 7..9 {
            for idx in row(r) {
                assert_eq!(b.side(idx), Side::Black, "Expect black piece at {}", idx);
            }
        }
        for r in 3..7 {
            for idx in row(r) {
                assert_eq!(b.side(idx), Side::Empty, "Expect empty side at {}", idx);
                assert_eq!(b.piece(idx), Piece::Empty, "Expect empty piece at {}", idx);
            }
        }
        for idx in vec![E1, E8] {
            assert_eq!(b.piece(idx), Piece::King, "Expect king at {}", idx);
        }
        for idx in vec![D1, D8] {
            assert_eq!(b.piece(idx), Piece::Queen, "Expect queen at {}", idx);
        }
        for idx in vec![A1, H1, A8, H8] {
            assert_eq!(b.piece(idx), Piece::Rook, "Expect rook at {}", idx);
        }
        for idx in vec![B1, G1, B8, G8] {
            assert_eq!(b.piece(idx), Piece::Knight, "Expect knight at {}", idx);
        }
        for idx in vec![C1, F1, C8, F8] {
            assert_eq!(b.piece(idx), Piece::Bishop, "Expect bishop at {}", idx);
        }
        for idx in row(2) {
            assert_eq!(b.piece(idx), Piece::Pawn, "Expect pawn at {}", idx);
        }
        for idx in row(7) {
            assert_eq!(b.piece(idx), Piece::Pawn, "Expect pawn at {}", idx);
        }
        for idx in fields() {
            let is_empty_piece = b.piece(idx) == Piece::Empty;
            let is_empty_side = b.side(idx) == Side::Empty;
            assert_eq!(
                is_empty_piece, is_empty_side,
                "Expect empty side <=> emty piece at {}",
                idx
            );
        }
    }

    #[test]
    fn from() {
        let b = Board::from(
            &[(A1, Side::White, Piece::Rook)],
            Side::White,
            [true, true, true, true],
            None,
            0,
            1,
        );
        assert_eq!(b.side(A1), Side::White);
        assert_eq!(b.piece(A1), Piece::Rook);
        assert_eq!(b.side(A2), Side::Empty);
        assert_eq!(b.piece(A2), Piece::Empty);
    }

    #[test]
    fn from2() {
        let b = Board::from(
            &[
                (H7, Side::White, Piece::Pawn),
                (H8, Side::Black, Piece::King),
            ],
            Side::White,
            [true, true, true, true],
            None,
            0,
            1,
        );
        assert_eq!(b.side(H7), Side::White);
        assert_eq!(b.piece(H7), Piece::Pawn);
        assert_eq!(b.side(H8), Side::Black);
        assert_eq!(b.piece(H8), Piece::King);
    }
}
