use crate::field::Field;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct MoveData {
    pub from: Field,
    pub to: Field,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Move {
    Capture(MoveData),
    Move(MoveData),
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
                Some(x) => match x.parse::<MoveData>() {
                    Ok(m) => Ok(Move::Capture(m)),
                    _ => Err(()),
                },
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

impl FromStr for MoveData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            Err(())?
        }
        let from = s[0..2].parse::<Field>()?;
        let to = s[2..4].parse::<Field>()?;
        Ok(MoveData { from, to })
    }
}

impl fmt::Display for MoveData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.from, self.to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::named::*;

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
}
