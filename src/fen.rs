use crate::board::Board;
use crate::field::{row, Field};
use crate::piece::ColoredPiece;
use std::convert::TryFrom;
use std::fmt::Write;

fn to_fen(b: &Board) -> String {
    let mut rv = String::new();
    for r in (1..9).rev() {
        let mut empty_count: u8 = 0;
        for f in row(r) {
            match b.pieces[f.0] {
                ColoredPiece::Empty => {
                    empty_count += 1;
                    continue;
                }
                e => {
                    if empty_count > 0 {
                        write!(&mut rv, "{}", empty_count).expect("Convert number to string is ok");
                        empty_count = 0;
                    }
                    if let Some(sym) = e.symbol() {
                        rv.push(sym);
                    }
                }
            }
        }
        if empty_count > 0 {
            write!(&mut rv, "{}", empty_count).expect("Convert number to string is ok");
        }
        rv.push(if r > 1 { '/' } else { ' ' });
    }
    rv.push(b.active.symbol());
    rv.push(' ');
    if b.can_castle[0] {
        rv.push('K');
    };
    if b.can_castle[1] {
        rv.push('Q');
    };
    if b.can_castle[2] {
        rv.push('k');
    };
    if b.can_castle[3] {
        rv.push('q');
    };
    if b.can_castle.iter().all(|x| !x) {
        rv.push('-');
    }
    rv.push(' ');
    match b.en_passant {
        Some(f) => write!(&mut rv, "{}", f).expect("Convert from field to string is ok"),
        None => rv.push('-'),
    }
    write!(&mut rv, " {} {}", b.halfmove_clock, b.full_moves)
        .expect("Convert from number to string is ok");
    rv
}

fn from_fen(s: &str) -> Result<Board, &'static str> {
    let mut b = Board::empty();
    let data: Vec<&str> = s.split(' ').collect();
    if data.len() != 6 {
        Err("Fen should contain 6 fields")?;
    }

    let field = data[0];
    let color = data[1];
    let castle = data[2];
    let en_passant = data[3];
    let halfmove_clock = data[4];
    let full_moves = data[5];

    let mut row = 8;
    let mut column = 1;
    for ch in field.chars() {
        if ch == '/' {
            if column != 9 {
                Err("Not all columns are present")?
            }
            row -= 1;
            column = 1;
            continue;
        }
        if let Ok(count) = String::from(ch).parse::<usize>() {
            column += count;
            continue;
        }
        let piece = ColoredPiece::try_from(ch)?;
        let f = Field::new(row, column);
        b.pieces[f.0] = piece;
        column += 1;
    }
    if row != 1 {
        Err("Not all rows are present")?
    }
    if column != 9 {
        Err("Not all columns are present")?
    }

    b.active = color.parse()?;

    b.can_castle[0] = castle.find('K').is_some();
    b.can_castle[1] = castle.find('Q').is_some();
    b.can_castle[2] = castle.find('k').is_some();
    b.can_castle[3] = castle.find('q').is_some();

    b.en_passant = en_passant.parse::<Field>().ok();
    b.halfmove_clock = halfmove_clock
        .parse::<u32>()
        .map_err(|_x| "Invalid halfmove")?;
    b.full_moves = full_moves
        .parse::<u32>()
        .map_err(|_x| "Invalid fullmoves")?;

    Ok(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::named::*;
    use crate::piece::Side;

    #[test]
    fn from_string1() {
        let parsed = from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let b = Board::initial();
        assert_eq!(Ok(b), parsed);
    }

    #[test]
    fn from_string2() {
        let parsed = from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b K-kq a3 11 13");
        let mut b = Board::initial();
        b.active = Side::Black;
        b.can_castle[1] = false;
        b.en_passant = Some(A3);
        b.halfmove_clock = 11;
        b.full_moves = 13;
        assert_eq!(Ok(b), parsed);
    }

    #[test]
    fn from_string3() {
        let parsed = from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        let mut b = Board::initial();
        b.pieces.swap(E2.0, E4.0);
        b.active = Side::Black;
        b.en_passant = Some(E3);
        b.halfmove_clock = 0;
        b.full_moves = 1;
        assert_eq!(Ok(b), parsed);
    }

    #[test]
    fn from_string4() {
        let parsed = from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2");
        let mut b = Board::initial();
        b.pieces.swap(E2.0, E4.0);
        b.pieces.swap(D7.0, D5.0);
        b.active = Side::White;
        b.en_passant = Some(D6);
        b.halfmove_clock = 0;
        b.full_moves = 2;
        assert_eq!(Ok(b), parsed);
    }

    #[test]
    fn from_string5() {
        let parsed = from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
        let mut b = Board::initial();
        b.pieces.swap(E2.0, E4.0);
        b.pieces.swap(D7.0, D5.0);
        b.pieces.swap(G1.0, F3.0);
        b.active = Side::Black;
        b.en_passant = None;
        b.halfmove_clock = 1;
        b.full_moves = 2;
        assert_eq!(Ok(b), parsed);
    }

    #[test]
    fn from_string6() {
        let parsed = from_fen("rnbq1bnr/pppkpppp/8/3p4/4P3/5N2/PPPP1PPP/RNBQKB1R w KQ - 2 3");
        let mut b = Board::initial();
        b.pieces.swap(E2.0, E4.0);
        b.pieces.swap(D7.0, D5.0);
        b.pieces.swap(G1.0, F3.0);
        b.pieces.swap(E8.0, D7.0);
        b.active = Side::White;
        b.en_passant = None;
        b.can_castle[2] = false;
        b.can_castle[3] = false;
        b.halfmove_clock = 2;
        b.full_moves = 3;
        assert_eq!(Ok(b), parsed);
    }

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
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Qkq - 0 1"
        );
        b.can_castle[2] = false;
        assert_eq!(
            to_fen(&b),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Qq - 0 1"
        );
        b.can_castle[3] = false;
        assert_eq!(
            to_fen(&b),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Q - 0 1"
        );
        b.can_castle[1] = false;
        assert_eq!(
            to_fen(&b),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1"
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
