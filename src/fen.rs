use crate::board::Board;
use crate::field::row;
use crate::piece::Piece;
use crate::side::Side;
use std::fmt::Write;

fn to_fen(b: &Board) -> String {
    let mut rv = String::new();
    for r in (1..9).rev() {
        let mut empty_count: u8 = 0;
        for f in row(r) {
            let p = b.piece(f);
            let s = b.side(f);
            if let Piece::Empty = p {
                empty_count += 1;
                continue;
            }
            if empty_count > 0 {
                write!(&mut rv, "{}", empty_count).expect("Convert number to string is ok");
                empty_count = 0;
            }
            let sym = match (p, s) {
                (Piece::King, Side::Black) => 'k',
                (Piece::Queen, Side::Black) => 'q',
                (Piece::Rook, Side::Black) => 'r',
                (Piece::Knight, Side::Black) => 'n',
                (Piece::Bishop, Side::Black) => 'b',
                (Piece::Pawn, Side::Black) => 'p',
                (Piece::King, Side::White) => 'K',
                (Piece::Queen, Side::White) => 'Q',
                (Piece::Rook, Side::White) => 'R',
                (Piece::Knight, Side::White) => 'N',
                (Piece::Bishop, Side::White) => 'B',
                (Piece::Pawn, Side::White) => 'P',
                _ => panic!("Unknown piece={:?} side={:?}", p, s),
            };
            rv.push(sym);
        }
        if empty_count > 0 {
            write!(&mut rv, "{}", empty_count).expect("Convert number to string is ok");
        }
        rv.push(if r > 1 { '/' } else { ' ' });
    }
    match b.active {
        Side::White => rv.push('w'),
        Side::Black => rv.push('b'),
        _ => panic!("Non-initialized board side"),
    }
    rv.push(' ');
    rv.push(if b.can_castle[0] { 'K' } else { '-' });
    rv.push(if b.can_castle[1] { 'Q' } else { '-' });
    rv.push(if b.can_castle[2] { 'k' } else { '-' });
    rv.push(if b.can_castle[3] { 'q' } else { '-' });
    rv.push(' ');
    match b.en_passant {
        Some(f) => write!(&mut rv, "{}", f).expect("Convert from field to string is ok"),
        None => rv.push('-'),
    }
    write!(&mut rv, " {} {}", b.halfmove_clock, b.full_moves)
        .expect("Convert from number to string is ok");
    rv
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::named::*;

    #[test]
    fn to_string1() {
        let b = Board::initial();
        assert_eq!(
            to_fen(&b),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn to_string2() {
        let mut b = Board::initial();
        b.can_castle[0] = false;
        assert_eq!(
            to_fen(&b),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w -Qkq - 0 1"
        );
        b.can_castle[2] = false;
        assert_eq!(
            to_fen(&b),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w -Q-q - 0 1"
        );
        b.can_castle[3] = false;
        assert_eq!(
            to_fen(&b),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w -Q-- - 0 1"
        );
        b.can_castle[1] = false;
        assert_eq!(
            to_fen(&b),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w ---- - 0 1"
        );
    }

    #[test]
    fn to_string3() {
        let mut b = Board::initial();
        b.active = Side::Black;
        assert_eq!(
            to_fen(&b),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1"
        );
        b.halfmove_clock = 12;
        assert_eq!(
            to_fen(&b),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 12 1"
        );
        b.full_moves = 11;
        assert_eq!(
            to_fen(&b),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 12 11"
        );
        b.en_passant = Some(A3);
        assert_eq!(
            to_fen(&b),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq a3 12 11"
        );
    }
}
