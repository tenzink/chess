use crate::field::{named::*, Field, COUNT};
use crate::piece::{ColoredPiece, Side};

#[derive(Debug, PartialEq)]
pub struct Board {
    pub pieces: [ColoredPiece; COUNT],
    pub active: Side,
    pub can_castle: [bool; 4], // white-king, white-queen, black-king, black-queen
    pub en_passant: Option<Field>,
    pub halfmove_clock: u32,
    pub full_moves: u32,
}

impl Board {
    pub fn new() -> Board {
        Board::from(&[], Side::White, [true, true, true, true], None, 0, 1)
    }

    pub fn from(
        list: &[(Field, ColoredPiece)],
        active: Side,
        can_castle: [bool; 4],
        en_passant: Option<Field>,
        halfmove_clock: u32,
        full_moves: u32,
    ) -> Board {
        let mut pieces = [ColoredPiece::Empty; COUNT];
        for (idx, piece) in list {
            pieces[idx.0] = *piece;
        }
        Board {
            pieces,
            active,
            can_castle,
            en_passant,
            halfmove_clock,
            full_moves,
        }
    }

    pub fn initial() -> Board {
        use crate::piece::named;
        const LIST: [(Field, ColoredPiece); 32] = [
            (A1, named::R),
            (B1, named::N),
            (C1, named::B),
            (D1, named::Q),
            (E1, named::K),
            (F1, named::B),
            (G1, named::N),
            (H1, named::R),
            (A2, named::P),
            (B2, named::P),
            (C2, named::P),
            (D2, named::P),
            (E2, named::P),
            (F2, named::P),
            (G2, named::P),
            (H2, named::P),
            (A7, named::p),
            (B7, named::p),
            (C7, named::p),
            (D7, named::p),
            (E7, named::p),
            (F7, named::p),
            (G7, named::p),
            (H7, named::p),
            (A8, named::r),
            (B8, named::n),
            (C8, named::b),
            (D8, named::q),
            (E8, named::k),
            (F8, named::b),
            (G8, named::n),
            (H8, named::r),
        ];
        Board::from(&LIST, Side::White, [true, true, true, true], None, 0, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{fields, row};
    use crate::piece::named;

    #[test]
    fn new() {
        let board = Board::new();
        for i in fields() {
            assert_eq!(board.pieces[i.0], ColoredPiece::Empty);
        }
    }

    #[test]
    fn initial() {
        let board = Board::initial();
        for r in 1..3 {
            for idx in row(r) {
                assert!(
                    board.pieces[idx.0].is_white(),
                    "Expect white piece at {}",
                    idx
                );
            }
        }
        for r in 7..9 {
            for idx in row(r) {
                assert!(
                    board.pieces[idx.0].is_black(),
                    "Expect black piece at {}",
                    idx
                );
            }
        }
        for r in 3..7 {
            for idx in row(r) {
                assert_eq!(
                    board.pieces[idx.0],
                    ColoredPiece::Empty,
                    "Expect empty at {}",
                    idx
                );
            }
        }
        assert_eq!(board.pieces[E1.0], named::K);
        assert_eq!(board.pieces[E8.0], named::k);
        assert_eq!(board.pieces[D1.0], named::Q);
        assert_eq!(board.pieces[D8.0], named::q);
        assert_eq!(board.pieces[A1.0], named::R);
        assert_eq!(board.pieces[H1.0], named::R);
        assert_eq!(board.pieces[A8.0], named::r);
        assert_eq!(board.pieces[H8.0], named::r);
        assert_eq!(board.pieces[B1.0], named::N);
        assert_eq!(board.pieces[G1.0], named::N);
        assert_eq!(board.pieces[B8.0], named::n);
        assert_eq!(board.pieces[G8.0], named::n);
        assert_eq!(board.pieces[C1.0], named::B);
        assert_eq!(board.pieces[F1.0], named::B);
        assert_eq!(board.pieces[C8.0], named::b);
        assert_eq!(board.pieces[F8.0], named::b);
        for idx in row(2) {
            assert_eq!(board.pieces[idx.0], named::P);
        }
        for idx in row(7) {
            assert_eq!(board.pieces[idx.0], named::p);
        }
    }

    #[test]
    fn from() {
        let board = Board::from(
            &[(A1, named::R)],
            Side::White,
            [true, true, true, true],
            None,
            0,
            1,
        );
        assert_eq!(board.pieces[A1.0], named::R);
        assert_eq!(board.pieces[A2.0], ColoredPiece::Empty);
    }

    #[test]
    fn from2() {
        let board = Board::from(
            &[(H7, named::P), (H8, named::k)],
            Side::White,
            [true, true, true, true],
            None,
            0,
            1,
        );
        assert_eq!(board.pieces[H7.0], named::P);
        assert_eq!(board.pieces[H8.0], named::k);
    }
}
