use crate::board::Board;
use crate::field::{fields, Field};
use crate::mv::{capture, mv, Move};
use crate::piece::{ColoredPiece, Piece, Side};

pub fn generate(b: &Board) -> Vec<Move> {
    let mut rv: Vec<Move> = Vec::new();
    for idx in fields() {
        if let ColoredPiece::P(piece, s) = b.pieces[idx.0] {
            if s != b.active {
                continue;
            }
            let mut piece_moves = |moves, slide| gen_piece_moves(idx, b, moves, slide, &mut rv);
            match piece {
                Piece::King => piece_moves(&[-11, -10, -9, -1, 1, 9, 10, 11], false),
                Piece::Queen => piece_moves(&[-11, -10, -9, -1, 1, 9, 10, 11], true),
                Piece::Rook => piece_moves(&[-10, -1, 1, 10], true),
                Piece::Bishop => piece_moves(&[-11, -9, 9, 11], true),
                Piece::Knight => piece_moves(&[-21, -19, -12, -8, 8, 12, 19, 21], false),
                Piece::Pawn => {
                    let mult = match b.active {
                        Side::White => 1,
                        Side::Black => -1,
                    };
                    let (initial, promotes) = match b.active {
                        Side::White => (idx.row() == 2, idx.row() == 7),
                        Side::Black => (idx.row() == 7, idx.row() == 2),
                    };
                    let captures = &[9 * mult, 11 * mult];
                    let promotes = if promotes {
                        vec![
                            Some(Piece::Queen),
                            Some(Piece::Rook),
                            Some(Piece::Bishop),
                            Some(Piece::Knight),
                        ]
                    } else {
                        vec![None]
                    };
                    if initial {
                        let moves = &[10 * mult, 10 * mult];
                        gen_pawn_moves(idx, b, moves, captures, &promotes, &mut rv);
                    } else {
                        let moves = &[10 * mult];
                        gen_pawn_moves(idx, b, moves, captures, &promotes, &mut rv);
                    }
                }
            }
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

fn move64(idx64: usize, offset120: isize) -> Option<usize> {
    let rv = MAILBOX_INDICES[(MAILBOX120_INDICES[idx64] + offset120) as usize];
    if rv == MX {
        None
    } else {
        Some(rv)
    }
}

fn gen_pawn_moves(
    idx: Field,
    b: &Board,
    move_offsets: &[isize],
    capture_offsets: &[isize],
    promotes: &[Option<Piece>],
    rv: &mut Vec<Move>,
) {
    let mut n = idx.0;
    for offset in move_offsets {
        n = match move64(n, *offset) {
            Some(rv) => rv,
            None => continue,
        };
        let f = Field::from(n);
        if ColoredPiece::Empty != b.pieces[f.0] {
            break;
        }
        for p in promotes {
            rv.push(mv(idx, f, *p));
        }
    }
    let n = idx.0;
    for offset in capture_offsets {
        let f = match move64(n, *offset) {
            Some(rv) => Field::from(rv),
            None => continue,
        };
        if b.en_passant == Some(f) {
            for p in promotes {
                rv.push(capture(idx, f, *p));
            }
        } else if let ColoredPiece::P(_, c) = b.pieces[f.0] {
            if c != b.active {
                for p in promotes {
                    rv.push(capture(idx, f, *p));
                }
            }
        }
    }
}

fn gen_piece_moves(idx: Field, b: &Board, offsets: &[isize], is_sliding: bool, rv: &mut Vec<Move>) {
    for offset in offsets {
        let mut n = idx.0;
        loop {
            n = match move64(n, *offset) {
                Some(rv) => rv,
                None => break,
            };
            let f = Field::from(n);
            if let ColoredPiece::P(_, c) = b.pieces[f.0] {
                if c != b.active {
                    rv.push(capture(idx, f, None));
                }
                break;
            }
            rv.push(mv(idx, f, None));
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
    use crate::field::named;
    use crate::piece::ColoredPiece;
    use crate::piece::Side;
    use crate::piece::Side::*;
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
        let b = Board::empty();
        let mv = generate(&b);
        assert_eq!(mv.len(), 0);
    }

    fn piece(s: &str) -> (Field, ColoredPiece) {
        let piece = s[..1].parse::<ColoredPiece>().unwrap();
        let pos = s[1..].parse::<Field>().unwrap();
        (pos, piece)
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
        let b = Board::new(&pieces, side, can_castle, en_passant, 0, 1);
        let moves = generate(&b);
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
        test_moves(
            White,
            &["Ka1", "Pa2", "pb2"],
            &["a1b1", "a1xb2", "a2a3", "a2a4"],
            [true, true, true, true],
            None,
        );
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

    #[test]
    fn pawn() {
        test_moves(
            White,
            &["Pa7"],
            &["a7a8=Q", "a7a8=R", "a7a8=B", "a7a8=N"],
            [true, true, true, true],
            None,
        );
        test_moves(
            Black,
            &["pa2"],
            &["a2a1=Q", "a2a1=R", "a2a1=B", "a2a1=N"],
            [true, true, true, true],
            None,
        );
        test_moves(
            White,
            &["Pa7", "ra8", "bb8"],
            &["a7xb8=Q", "a7xb8=R", "a7xb8=B", "a7xb8=N"],
            [true, true, true, true],
            None,
        );
        test_moves(
            Black,
            &["pa2", "Ra1", "Bb1"],
            &["a2xb1=Q", "a2xb1=R", "a2xb1=B", "a2xb1=N"],
            [true, true, true, true],
            None,
        );
        test_moves(
            White,
            &["Pe2"],
            &["e2e3", "e2e4"],
            [true, true, true, true],
            None,
        );
        test_moves(
            Black,
            &["pe7"],
            &["e7e6", "e7e5"],
            [true, true, true, true],
            None,
        );
        test_moves(
            White,
            &["Pd2", "pc3", "pe3"],
            &["d2d3", "d2d4", "d2xc3", "d2xe3"],
            [true, true, true, true],
            None,
        );
        test_moves(
            Black,
            &["pd7", "Pc6", "Pe6"],
            &["d7d6", "d7d5", "d7xc6", "d7xe6"],
            [true, true, true, true],
            None,
        );
        test_moves(
            White,
            &["Pd6", "pe6"],
            &["d6d7", "d6xe7"],
            [true, true, true, true],
            Some(named::E7),
        );
        test_moves(
            Black,
            &["pd4", "Pe4"],
            &["d4d3", "d4xe3"],
            [true, true, true, true],
            Some(named::E3),
        );
        test_moves(White, &["Pd4"], &["d4d5"], [true, true, true, true], None);
        test_moves(Black, &["pd4"], &["d4d3"], [true, true, true, true], None);
    }
}
