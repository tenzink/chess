use crate::board::Board;

fn to_fen(_b: &Board) -> String {
    let mut s = String::new();
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let b = Board::initial();
        assert_eq!(to_fen(&b), "");
    }
}
