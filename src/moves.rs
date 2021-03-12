use crate::board::Board;
use crate::field::{fields, Field};
use crate::mv::{capture, mv, Move};
use crate::piece::Piece;

pub fn moves(b: &Board) -> Vec<Move> {
    let mut rv: Vec<Move> = Vec::new();
    for idx in fields() {
        if b.side(idx) != b.active {
            continue;
        }
        let piece = b.piece(idx);
        let mut gen_moves = |moves, slide| moves_iml(idx, b, moves, slide, &mut rv);
        match piece {
            Piece::Empty => continue,
            Piece::King => gen_moves(&[-11, -10, -9, -1, 1, 9, 10, 11], false),
            Piece::Queen => gen_moves(&[-11, -10, -9, -1, 1, 9, 10, 11], true),
            Piece::Rook => gen_moves(&[-10, -1, 1, 10], true),
            Piece::Bishop => gen_moves(&[-11, -9, 9, 11], true),
            Piece::Knight => gen_moves(&[-21, -19, -12, -8, 8, 12, 19, 21], false),
            Piece::Pawn => continue,
        }
    }
    rv
}

///////////////////////////////////////////////////////////////////////////////

const MX: usize = usize::MAX;

#[rustfmt::skip]
const MAILBOX_INDICES: [usize; 120] = [ 
    MX, MX, MX, MX, MX, MX, MX, MX, MX, MX,
    MX, MX, MX, MX, MX, MX, MX, MX, MX, MX,
    MX,  0,  1,  2,  3,  4,  5,  6,  7, MX,
    MX,  8,  9, 10, 11, 12, 13, 14, 15, MX,
    MX, 16, 17, 18, 19, 20, 21, 22, 23, MX,
    MX, 24, 25, 26, 27, 28, 29, 30, 31, MX,
    MX, 32, 33, 34, 35, 36, 37, 38, 39, MX,
    MX, 40, 41, 42, 43, 44, 45, 46, 47, MX,
    MX, 48, 49, 50, 51, 52, 53, 54, 55, MX,
    MX, 56, 57, 58, 59, 60, 61, 62, 63, MX,
    MX, MX, MX, MX, MX, MX, MX, MX, MX, MX,
    MX, MX, MX, MX, MX, MX, MX, MX, MX, MX];

#[rustfmt::skip]
const MAILBOX120_INDICES: [isize; 64] = [ 
    21, 22, 23, 24, 25, 26, 27, 28,
    31, 32, 33, 34, 35, 36, 37, 38,
    41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58,
    61, 62, 63, 64, 65, 66, 67, 68,
    71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88,
    91, 92, 93, 94, 95, 96, 97, 98];

fn moves_iml(idx: Field, b: &Board, offsets: &[isize], is_sliding: bool, rv: &mut Vec<Move>) {
    for off in offsets {
        let mut n = idx.0;
        loop {
            n = MAILBOX_INDICES[(MAILBOX120_INDICES[n] + off) as usize];
            if n == MX {
                break;
            }
            let f = Field::from(n);
            if b.piece(f) != Piece::Empty {
                if b.side(f) != b.active {
                    rv.push(capture(idx, f));
                }
                break;
            }
            rv.push(mv(idx, f));
            if !is_sliding {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::piece::Piece::*;
    use crate::side::Side;
    use crate::side::Side::*;
    use std::collections::HashSet;

    #[test]
    fn mailboxes() {
        for (idx, &i) in MAILBOX_INDICES.iter().enumerate() {
            if i == MX {
                continue;
            }
            assert_eq!(MAILBOX120_INDICES[i] as usize, idx);
        }
        for (idx, &i) in MAILBOX120_INDICES.iter().enumerate() {
            assert_eq!(MAILBOX_INDICES[i as usize], idx);
        }
    }
    #[test]
    fn empty() {
        let b = Board::new();
        let mv = moves(&b);
        assert_eq!(mv.len(), 0);
    }

    fn piece(s: &str) -> (Field, Side, Piece) {
        let (side, piece) = match &s[..1] {
            "K" => (White, King),
            "k" => (Black, King),
            "Q" => (White, Queen),
            "q" => (Black, Queen),
            "R" => (White, Rook),
            "r" => (Black, Rook),
            "B" => (White, Bishop),
            "b" => (Black, Bishop),
            "N" => (White, Knight),
            "n" => (Black, Knight),
            "P" => (White, Pawn),
            "p" => (Black, Pawn),
            _ => panic!("Unknown piece"),
        };
        let pos = s[1..].parse::<Field>().unwrap();
        (pos, side, piece)
    }

    fn test_moves(
        side: Side,
        pieces_str: &[&str],
        expected: &[&str],
        can_castle: [bool; 4],
        en_passant: Option<Field>,
    ) {
        let mut expected_moves = HashSet::<Move>::new();
        for mv in expected {
            let mv: Move = mv.parse::<Move>().unwrap();
            expected_moves.insert(mv);
        }
        let mut pieces = Vec::new();
        for pieces_str in pieces_str {
            pieces.push(piece(pieces_str));
        }
        let b = Board::from(&pieces, side, can_castle, en_passant, 0, 1);
        let moves = moves(&b);
        let moves: HashSet<_> = moves.iter().cloned().collect();
        let expected = expected_moves;
        let redundant: Vec<_> = moves.difference(&expected).collect();
        let not_found: Vec<_> = expected.difference(&moves).collect();
        for m in &redundant {
            println!("Redundant: {}", m);
        }
        for m in &not_found {
            println!("Not found: {}", m);
        }
        assert_eq!(moves, expected);
    }

    #[test]
    fn king() {
        test_moves(
            White,
            &["Ka1"],
            &["a1a2", "a1b1", "a1b2"],
            [true, true, true, true],
            None,
        );
    }

    #[test]
    fn king2() {
        test_moves(
            White,
            &["Ka1", "Pa2", "pb2"],
            &["a1b1", "a1b2x"],
            [true, true, true, true],
            None,
        );
    }

    #[test]
    fn king3() {
        test_moves(
            White,
            &["Ke4"],
            &[
                "e4e5", "e4e3", "e4f3", "e4f4", "e4f5", "e4d3", "e4d4", "e4d5",
            ],
            [true, true, true, true],
            None,
        );
    }

    #[test]
    fn queen() {
        test_moves(
            White,
            &["Qa1"],
            &[
                "a1a2", "a1a3", "a1a4", "a1a5", "a1a6", "a1a7", "a1a8", "a1b1", "a1c1", "a1d1",
                "a1e1", "a1f1", "a1g1", "a1h1", "a1b2", "a1c3", "a1d4", "a1e5", "a1f6", "a1g7",
                "a1h8",
            ],
            [true, true, true, true],
            None,
        );
    }

    #[test]
    fn rook() {
        test_moves(
            White,
            &["Rb2"],
            &[
                "b2a2", "b2c2", "b2d2", "b2e2", "b2f2", "b2g2", "b2h2", "b2b1", "b2b3", "b2b4",
                "b2b5", "b2b6", "b2b7", "b2b8",
            ],
            [true, true, true, true],
            None,
        );
    }

    #[test]
    fn bishop() {
        test_moves(
            White,
            &["Bc2"],
            &[
                "c2b1", "c2d3", "c2e4", "c2f5", "c2g6", "c2h7", "c2b3", "c2a4", "c2d1",
            ],
            [true, true, true, true],
            None,
        );
    }

    #[test]
    fn knight() {
        test_moves(
            White,
            &["Nh8"],
            &["h8g6", "h8f7"],
            [true, true, true, true],
            None,
        );
    }

    #[test]
    fn knight2() {
        test_moves(
            White,
            &["Nd4"],
            &[
                "d4c2", "d4e2", "d4b3", "d4f3", "d4c6", "d4e6", "d4b5", "d4f5",
            ],
            [true, true, true, true],
            None,
        );
    }
}
