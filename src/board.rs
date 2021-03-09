use crate::field::*;
use crate::piece::Piece;
use crate::side::Side;

#[cfg(test)]
mod tests {
    use super::*;

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
        let whites = vec![
            A1, B1, C1, D1, E1, F1, G1, H1, A2, B2, C2, D2, E2, F2, G2, H2,
        ];
        for idx in whites {
            assert_eq!(b.side(idx), Side::White, "Expect white piece at {}", idx);
        }
        let blacks = vec![
            A7, B7, C7, D7, E7, F7, G7, H7, A8, B8, C8, D8, E8, F8, G8, H8,
        ];
        for idx in blacks {
            assert_eq!(b.side(idx), Side::Black, "Expect black piece at {}", idx);
        }
        let empty = vec![
            A3, B3, C3, D3, E3, F3, G3, H3, A4, B4, C4, D4, E4, F4, G4, H4, A5, B5, C5, D5, E5, F5,
            G5, H5, A6, B6, C6, D6, E6, F6, G6, H6,
        ];
        for idx in empty {
            assert_eq!(b.side(idx), Side::Empty, "Expect empty piece at {}", idx);
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
        for idx in vec![A2, B2, C2, D2, E2, F2, G2, H2] {
            assert_eq!(b.piece(idx), Piece::Pawn, "Expect pawn at {}", idx);
        }
        for idx in vec![A7, B7, C7, D7, E7, F7, G7, H7] {
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
        let b = Board::from(&[(A1, Side::White, Piece::Rook)]);
        assert_eq!(b.side(A1), Side::White);
        assert_eq!(b.piece(A1), Piece::Rook);
        assert_eq!(b.side(A2), Side::Empty);
        assert_eq!(b.piece(A2), Piece::Empty);
    }

    #[test]
    fn from2() {
        let b = Board::from(&[
            (H7, Side::White, Piece::Pawn),
            (H8, Side::Black, Piece::King),
        ]);
        assert_eq!(b.side(H7), Side::White);
        assert_eq!(b.piece(H7), Piece::Pawn);
        assert_eq!(b.side(H8), Side::Black);
        assert_eq!(b.piece(H8), Piece::King);
    }
}

pub struct Board {
    sides: [Side; COUNT],
    pieces: [Piece; COUNT],
}

impl Board {
    pub fn side(&self, f: Field) -> Side {
        self.sides[f.0]
    }

    pub fn piece(&self, f: Field) -> Piece {
        self.pieces[f.0]
    }

    pub fn new() -> Board {
        Board {
            sides: [Side::Empty; COUNT],
            pieces: [Piece::Empty; COUNT],
        }
    }
    pub fn from(list: &[(Field, Side, Piece)]) -> Board {
        let mut sides = [Side::Empty; COUNT];
        let mut pieces = [Piece::Empty; COUNT];
        for (idx, side, piece) in list {
            sides[idx.0] = *side;
            pieces[idx.0] = *piece;
        }
        Board { sides, pieces }
    }
    pub fn initial() -> Board {
        const LIST: [(Field, Side, Piece); 32] = [
            (A1, Side::White, Piece::Rook),
            (B1, Side::White, Piece::Knight),
            (C1, Side::White, Piece::Bishop),
            (D1, Side::White, Piece::Queen),
            (E1, Side::White, Piece::King),
            (F1, Side::White, Piece::Bishop),
            (G1, Side::White, Piece::Knight),
            (H1, Side::White, Piece::Rook),
            (A2, Side::White, Piece::Pawn),
            (B2, Side::White, Piece::Pawn),
            (C2, Side::White, Piece::Pawn),
            (D2, Side::White, Piece::Pawn),
            (E2, Side::White, Piece::Pawn),
            (F2, Side::White, Piece::Pawn),
            (G2, Side::White, Piece::Pawn),
            (H2, Side::White, Piece::Pawn),
            (A7, Side::Black, Piece::Pawn),
            (B7, Side::Black, Piece::Pawn),
            (C7, Side::Black, Piece::Pawn),
            (D7, Side::Black, Piece::Pawn),
            (E7, Side::Black, Piece::Pawn),
            (F7, Side::Black, Piece::Pawn),
            (G7, Side::Black, Piece::Pawn),
            (H7, Side::Black, Piece::Pawn),
            (A8, Side::Black, Piece::Rook),
            (B8, Side::Black, Piece::Knight),
            (C8, Side::Black, Piece::Bishop),
            (D8, Side::Black, Piece::Queen),
            (E8, Side::Black, Piece::King),
            (F8, Side::Black, Piece::Bishop),
            (G8, Side::Black, Piece::Knight),
            (H8, Side::Black, Piece::Rook),
        ];
        Board::from(&LIST)
    }
}
