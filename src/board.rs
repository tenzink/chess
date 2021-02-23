use crate::field::*;
use crate::piece::Piece;
use crate::side::Side;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let b = Board::new();
        for i in A1..COUNT {
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
            assert_eq!(b.side(idx), Side::White);
        }
        let blacks = vec![
            A7, B7, C7, D7, E7, F7, G7, H7, A8, B8, C8, D8, E8, F8, G8, H8,
        ];
        for idx in blacks {
            assert_eq!(b.side(idx), Side::Black);
        }
        let empty = vec![
            A3, B3, C3, D3, E3, F3, G3, H3, A4, B4, C4, D4, E4, F4, G4, H4, A5, B5, C5, D5, E5, F5,
            G5, H5, A6, B6, C6, D6, E6, F6, G6, H6,
        ];
        for idx in empty {
            assert_eq!(b.side(idx), Side::Empty);
        }
        for idx in vec![E1, E8] {
            assert_eq!(b.piece(idx), Piece::King);
        }
        for idx in vec![D1, D8] {
            assert_eq!(b.piece(idx), Piece::Queen);
        }
        for idx in vec![A1, H1, A8, H8] {
            assert_eq!(b.piece(idx), Piece::Rook);
        }
        for idx in vec![B1, G1, B8, G8] {
            assert_eq!(b.piece(idx), Piece::Knight);
        }
        for idx in vec![C1, F1, C8, F8] {
            assert_eq!(b.piece(idx), Piece::Bishop);
        }
        for idx in vec![A2, B2, C2, D2, E2, F2, G2, H2] {
            assert_eq!(b.piece(idx), Piece::Pawn);
        }
        for idx in vec![A7, B7, C7, D7, E7, F7, G7, H7] {
            assert_eq!(b.piece(idx), Piece::Pawn);
        }
        for idx in 0..COUNT {
            let is_empty_piece = b.piece(idx) == Piece::Empty;
            let is_empty_side = b.side(idx) == Side::Empty;
            assert_eq!(is_empty_piece, is_empty_side);
        }
    }
}

pub struct Board {
    sides: [Side; COUNT],
    pieces: [Piece; COUNT],
}

impl Board {
    pub fn side(&self, idx: usize) -> Side {
        self.sides[idx]
    }

    pub fn piece(&self, idx: usize) -> Piece {
        self.pieces[idx]
    }

    pub fn new() -> Board {
        Board {
            sides: [Side::Empty; COUNT],
            pieces: [Piece::Empty; COUNT],
        }
    }
    pub fn initial() -> Board {
        let mut sides = [Side::Empty; COUNT];
        sides[..A3].fill(Side::White);
        sides[A7..].fill(Side::Black);
        let mut pieces = [Piece::Empty; COUNT];
        pieces[A2..A3].fill(Piece::Pawn);
        pieces[A7..A8].fill(Piece::Pawn);
        const INITIAL: [Piece; 8] = [
            Piece::Rook,
            Piece::Knight,
            Piece::Bishop,
            Piece::Queen,
            Piece::King,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
        ];
        pieces[..A2].copy_from_slice(&INITIAL);
        pieces[A8..].copy_from_slice(&INITIAL);
        Board { sides, pieces }
    }
}
