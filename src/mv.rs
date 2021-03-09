use crate::field::Field;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct MoveData {
    from: Field,
    to: Field,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Move {
    Capture(MoveData),
    Move(MoveData),
}

pub fn mv(from: Field, to: Field) -> Move {
    Move::Move(MoveData { from, to })
}

pub fn capture(from: Field, to: Field) -> Move {
    Move::Capture(MoveData { from, to })
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Capture(m) => write!(f, "{}{}x", m.from, m.to),
            Move::Move(m) => write!(f, "{}{}", m.from, m.to),
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let from = s.get(0..2).ok_or_else(|| ())?.parse::<Field>()?;
        let to = s.get(2..4).ok_or_else(|| ())?.parse::<Field>()?;
        if s.len() == 4 {
            Ok(mv(from, to))
        } else if s.len() == 5 && Ok('x') == s.get(4..5).ok_or_else(|| ())?.parse::<char>() {
            Ok(capture(from, to))
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
        assert_eq!(mv(A1, B2).to_string(), "a1b2");
        assert_eq!(capture(D1, H7).to_string(), "d1h7x");
        assert_eq!(capture(E2, E4).to_string(), "e2e4x");
    }

    #[test]
    fn parse() {
        assert_eq!(Ok(mv(A1, B2)), "a1b2".parse::<Move>());
        assert_eq!(Ok(capture(D1, H7)), "d1h7x".parse::<Move>());
        assert_eq!(Ok(capture(E2, E4)), "e2e4x".parse::<Move>());
        assert!("e2e4uv".parse::<Move>().is_err());
        assert!("e2e4u".parse::<Move>().is_err());
        assert!("e2e".parse::<Move>().is_err());
        assert!("e2".parse::<Move>().is_err());
        assert!("e".parse::<Move>().is_err());
        assert!("".parse::<Move>().is_err());
    }
}
