use crate::board::Board;

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
        let mv = moves(&b);
        assert_eq!(mv.len(), 0);
    }
}

pub fn moves(b: &Board) -> Vec<Move> {
    let mut rv: Vec<Move> = Vec::new();
    rv
}
