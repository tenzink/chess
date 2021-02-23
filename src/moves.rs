use crate::board::Board;
use crate::field::*;
use crate::piece::Piece;
use crate::side::Side;

pub struct Move {
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
            Piece::King => continue,
            Piece::Queen => continue,
            Piece::Rook => continue,
            Piece::Bishop => continue,
            Piece::Knight => continue,
            Piece::Pawn => continue,
        }
    }
    rv
}
