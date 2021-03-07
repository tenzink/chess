use crate::board::Board;
use crate::field::*;
use crate::moves::from_algebraic;
use crate::piece::Piece;
use crate::side::Side;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Move {
    Capture(MoveData),
    Move(MoveData),
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Capture(m) => write!(f, "{}x", m),
            Move::Move(m) => write!(f, "{}", m),
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len == 0 {
            Err(())?
        }
        if len == 4 {
            match s.parse::<MoveData>() {
                Ok(m) => Ok(Move::Move(m)),
                _ => Err(()),
            }
        } else if len == 5 && s.ends_with("x") {
            let s = s.get(..4);
            match s {
                Some(x) => {
                    match x.parse::<MoveData>() {
                        Ok(m) => Ok(Move::Capture(m)),
                        _ => Err(())
                    }
                },
                _ => Err(())
            }
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct MoveData {
    from: usize,
    to: usize,
}

impl FromStr for MoveData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            Err(())?
        }
        match (from_algebraic(&s[0..2]), from_algebraic(&s[2..4])) {
            (Some(from), Some(to)) => Ok(MoveData { from, to }),
            _ => Err(()),
        }
    }
}

impl fmt::Display for MoveData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", algebraic(self.from), algebraic(self.to))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use std::collections::HashSet;

    #[test]
    fn move_data_to_str() {
        assert_eq!(MoveData { from: A1, to: B2 }.to_string(), "a1b2");
        assert_eq!(MoveData { from: D1, to: H7 }.to_string(), "d1h7");
        assert_eq!(MoveData { from: E2, to: E4 }.to_string(), "e2e4");
    }

    #[test]
    fn move_to_str() {
        assert_eq!(
            Move::Move(MoveData { from: A1, to: B2 }).to_string(),
            "a1b2"
        );
        assert_eq!(
            Move::Capture(MoveData { from: D1, to: H7 }).to_string(),
            "d1h7x"
        );
        assert_eq!(
            Move::Capture(MoveData { from: E2, to: E4 }).to_string(),
            "e2e4x"
        );
    }

    #[test]
    fn move_from_str() {
        assert_eq!(
            Ok(Move::Move(MoveData { from: A1, to: B2 })),
            "a1b2".parse::<Move>()
        );
                assert_eq!(
            Ok(Move::Capture(MoveData { from: D1, to: H7 })),
            "d1h7x".parse::<Move>()
        );
        assert_eq!(
            Ok(Move::Capture(MoveData { from: E2, to: E4 })),
            "e2e4x".parse::<Move>()
        );
    }
    #[test]
    fn move_data_from_str() {
        assert_eq!(
            Ok(MoveData { from: A1, to: B2 }),
            "a1b2".parse::<MoveData>()
        );
        assert_eq!(
            Ok(MoveData { from: D1, to: H7 }),
            "d1h7".parse::<MoveData>()
        );
        assert_eq!(
            Ok(MoveData { from: E2, to: E4 }),
            "e2e4".parse::<MoveData>()
        );
        assert!("e2e9".parse::<MoveData>().is_err());
        assert!("e2e4x".parse::<MoveData>().is_err());
        assert!("".parse::<MoveData>().is_err());
    }

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
        let mv = moves(&Side::White, &b);
        assert_eq!(mv.len(), 0);
    }

    fn test_moves(side: Side, pieces: &[(usize, Side, Piece)], expected: &[Move]) {
        let b = Board::from(pieces);
        let moves = moves(&side, &b);
        let moves: HashSet<_> = moves.iter().cloned().collect();
        let expected: HashSet<_> = expected.iter().cloned().collect();
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
            Side::White,
            &[(A1, Side::White, Piece::King)],
            &[
                Move::Move(MoveData { from: A1, to: A2 }),
                Move::Move(MoveData { from: A1, to: B1 }),
                Move::Move(MoveData { from: A1, to: B2 }),
            ],
        );
    }

    #[test]
    fn king2() {
        test_moves(
            Side::White,
            &[
                (A1, Side::White, Piece::King),
                (A2, Side::White, Piece::Pawn),
                (B2, Side::Black, Piece::Pawn),
            ],
            &[
                Move::Move(MoveData { from: A1, to: B1 }),
                Move::Capture(MoveData { from: A1, to: B2 }),
            ],
        );
    }

    #[test]
    fn king3() {
        test_moves(
            Side::White,
            &[(E4, Side::White, Piece::King)],
            &[
                Move::Move(MoveData { from: E4, to: E5 }),
                Move::Move(MoveData { from: E4, to: E3 }),
                Move::Move(MoveData { from: E4, to: F3 }),
                Move::Move(MoveData { from: E4, to: F4 }),
                Move::Move(MoveData { from: E4, to: F5 }),
                Move::Move(MoveData { from: E4, to: D3 }),
                Move::Move(MoveData { from: E4, to: D4 }),
                Move::Move(MoveData { from: E4, to: D5 }),
            ],
        );
    }

    #[test]
    fn queen() {
        test_moves(
            Side::White,
            &[(A1, Side::White, Piece::Queen)],
            &[
                Move::Move(MoveData { from: A1, to: A2 }),
                Move::Move(MoveData { from: A1, to: A3 }),
                Move::Move(MoveData { from: A1, to: A4 }),
                Move::Move(MoveData { from: A1, to: A5 }),
                Move::Move(MoveData { from: A1, to: A6 }),
                Move::Move(MoveData { from: A1, to: A7 }),
                Move::Move(MoveData { from: A1, to: A8 }),
                Move::Move(MoveData { from: A1, to: B1 }),
                Move::Move(MoveData { from: A1, to: C1 }),
                Move::Move(MoveData { from: A1, to: D1 }),
                Move::Move(MoveData { from: A1, to: E1 }),
                Move::Move(MoveData { from: A1, to: F1 }),
                Move::Move(MoveData { from: A1, to: G1 }),
                Move::Move(MoveData { from: A1, to: H1 }),
                Move::Move(MoveData { from: A1, to: B2 }),
                Move::Move(MoveData { from: A1, to: C3 }),
                Move::Move(MoveData { from: A1, to: D4 }),
                Move::Move(MoveData { from: A1, to: E5 }),
                Move::Move(MoveData { from: A1, to: F6 }),
                Move::Move(MoveData { from: A1, to: G7 }),
                Move::Move(MoveData { from: A1, to: H8 }),
            ],
        );
    }
}

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

fn moves_iml(
    idx: usize,
    side: &Side,
    b: &Board,
    offsets: &[isize],
    is_sliding: bool,
    rv: &mut Vec<Move>,
) {
    for off in offsets {
        let mut n = idx;
        loop {
            n = MAILBOX_INDICES[(MAILBOX120_INDICES[n] + off) as usize];
            if n == MX {
                break;
            }
            if b.piece(n) != Piece::Empty {
                if b.side(n) != *side {
                    rv.push(Move::Capture(MoveData { from: idx, to: n }));
                }
                break;
            }
            rv.push(Move::Move(MoveData { from: idx, to: n }));
            if !is_sliding {
                break;
            }
        }
    }
}

pub fn moves(side: &Side, b: &Board) -> Vec<Move> {
    let mut rv: Vec<Move> = Vec::new();
    for idx in 0..COUNT {
        if b.side(idx) != *side {
            continue;
        }
        let piece = b.piece(idx);
        match piece {
            Piece::Empty => continue,
            Piece::King => moves_iml(
                idx,
                side,
                b,
                &[-11, -10, -9, -1, 1, 9, 10, 11],
                false,
                &mut rv,
            ),
            Piece::Queen => moves_iml(
                idx,
                side,
                b,
                &[-11, -10, -9, -1, 1, 9, 10, 11],
                true,
                &mut rv,
            ),
            Piece::Rook => continue,
            Piece::Bishop => continue,
            Piece::Knight => continue,
            Piece::Pawn => continue,
        }
    }
    rv
}
