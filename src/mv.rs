use crate::field::Field;
use crate::piece::Piece;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct MoveData {
    from: Field,
    to: Field,
    promotion: Option<Piece>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Move {
    Capture(MoveData),
    Move(MoveData),
}

pub fn mv(from: Field, to: Field, promotion: Option<Piece>) -> Move {
    Move::Move(MoveData {
        from,
        to,
        promotion,
    })
}

pub fn capture(from: Field, to: Field, promotion: Option<Piece>) -> Move {
    Move::Capture(MoveData {
        from,
        to,
        promotion,
    })
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let capture_str = |c: Option<Piece>| match c {
            Option::Some(field) => format!("={}", field),
            None => "".to_string(),
        };
        match self {
            Move::Capture(m) => write!(f, "{}x{}{}", m.from, m.to, capture_str(m.promotion)),
            Move::Move(m) => write!(f, "{}{}{}", m.from, m.to, capture_str(m.promotion)),
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 4 {
            let from = s.get(0..2).ok_or_else(|| ())?.parse::<Field>()?;
            let to = s.get(2..4).ok_or_else(|| ())?.parse::<Field>()?;
            Ok(mv(from, to, None))
        } else if s.len() == 5 && Ok('x') == s.get(2..3).ok_or_else(|| ())?.parse::<char>() {
            let from = s.get(0..2).ok_or_else(|| ())?.parse::<Field>()?;
            let to = s.get(3..5).ok_or_else(|| ())?.parse::<Field>()?;
            Ok(capture(from, to, None))
        } else if s.len() == 6 && Ok('=') == s.get(4..5).ok_or_else(|| ())?.parse::<char>() {
            let from = s.get(0..2).ok_or_else(|| ())?.parse::<Field>()?;
            let to = s.get(2..4).ok_or_else(|| ())?.parse::<Field>()?;
            let piece = s
                .get(5..6)
                .ok_or_else(|| ())?
                .parse::<Piece>()
                .map_err(|_| ())?;
            Ok(mv(from, to, Some(piece)))
        } else if s.len() == 7
            && Ok('x') == s.get(2..3).ok_or_else(|| ())?.parse::<char>()
            && Ok('=') == s.get(5..6).ok_or_else(|| ())?.parse::<char>()
        {
            let from = s.get(0..2).ok_or_else(|| ())?.parse::<Field>()?;
            let to = s.get(3..5).ok_or_else(|| ())?.parse::<Field>()?;
            let piece = s
                .get(6..7)
                .ok_or_else(|| ())?
                .parse::<Piece>()
                .map_err(|_| ())?;
            Ok(capture(from, to, Some(piece)))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::named::*;

    #[test]
    fn to_string() {
        assert_eq!(mv(A1, B2, None).to_string(), "a1b2");
        assert_eq!(capture(D1, H7, None).to_string(), "d1xh7");
        assert_eq!(capture(E2, E4, None).to_string(), "e2xe4");
        assert_eq!(mv(A7, A8, Some(Piece::Queen)).to_string(), "a7a8=Q");
        assert_eq!(capture(C7, B8, Some(Piece::Knight)).to_string(), "c7xb8=N");
    }

    #[test]
    fn parse() {
        assert_eq!(Ok(mv(A1, B2, None)), "a1b2".parse::<Move>());
        assert_eq!(Ok(capture(D1, H7, None)), "d1xh7".parse::<Move>());
        assert_eq!(Ok(capture(E2, E4, None)), "e2xe4".parse::<Move>());
        assert_eq!(Ok(mv(A7, A8, Some(Piece::Queen))), "a7a8=Q".parse::<Move>());
        assert_eq!(
            Ok(capture(C7, B8, Some(Piece::Rook))),
            "c7xb8=R".parse::<Move>()
        );
        assert!("e2e4uv".parse::<Move>().is_err());
        assert!("e2e4u".parse::<Move>().is_err());
        assert!("e2e".parse::<Move>().is_err());
        assert!("e2".parse::<Move>().is_err());
        assert!("e".parse::<Move>().is_err());
        assert!("".parse::<Move>().is_err());
    }
}
