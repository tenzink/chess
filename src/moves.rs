use crate::board::Board;
use crate::field::*;
use crate::piece::Piece;
use crate::side::Side;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Move {
    Capture(MoveData),
    Move(MoveData),
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MoveData {
    from: usize,
    to: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn test_moves_on_empty_board() {
        let b = Board::new();
        let mv = moves(&Side::White, &b);
        assert_eq!(mv.len(), 0);
    }

    #[test]
    fn test_king_moves() {
        let b = Board::from(&[(A1, Side::White, Piece::King)]);
        let mut mv = moves(&Side::White, &b);
        let mut moves = vec![
            Move::Move(MoveData { from: A1, to: A2 }),
            Move::Move(MoveData { from: A1, to: B1 }),
            Move::Move(MoveData { from: A1, to: B2 }),
        ];
        mv.sort();
        moves.sort();
        assert_eq!(mv, moves);
    }

    #[test]
    fn test_king_moves2() {
        let b = Board::from(&[
            (A1, Side::White, Piece::King),
            (A2, Side::White, Piece::Pawn),
            (B2, Side::Black, Piece::Pawn),
        ]);
        let mut mv = moves(&Side::White, &b);
        let mut moves = vec![
            Move::Move(MoveData { from: A1, to: B1 }),
            Move::Capture(MoveData { from: A1, to: B2 }),
        ];
        mv.sort();
        moves.sort();
        assert_eq!(mv, moves);
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
        loop {
            let mut n = MAILBOX_INDICES[(MAILBOX120_INDICES[idx] + off) as usize];
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
            Piece::Queen => continue,
            Piece::Rook => continue,
            Piece::Bishop => continue,
            Piece::Knight => continue,
            Piece::Pawn => continue,
        }
    }
    rv
}
