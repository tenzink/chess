use std::fmt;
use std::option::Option;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Field(pub usize);

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", NAMES[self.0])
    }
}

pub const A1: Field = Field(0);
pub const B1: Field = Field(1);
pub const C1: Field = Field(2);
pub const D1: Field = Field(3);
pub const E1: Field = Field(4);
pub const F1: Field = Field(5);
pub const G1: Field = Field(6);
pub const H1: Field = Field(7);

pub const A2: Field = Field(8);
pub const B2: Field = Field(9);
pub const C2: Field = Field(10);
pub const D2: Field = Field(11);
pub const E2: Field = Field(12);
pub const F2: Field = Field(13);
pub const G2: Field = Field(14);
pub const H2: Field = Field(15);

pub const A3: Field = Field(16);
pub const B3: Field = Field(17);
pub const C3: Field = Field(18);
pub const D3: Field = Field(19);
pub const E3: Field = Field(20);
pub const F3: Field = Field(21);
pub const G3: Field = Field(22);
pub const H3: Field = Field(23);

pub const A4: Field = Field(24);
pub const B4: Field = Field(25);
pub const C4: Field = Field(26);
pub const D4: Field = Field(27);
pub const E4: Field = Field(28);
pub const F4: Field = Field(29);
pub const G4: Field = Field(30);
pub const H4: Field = Field(31);

pub const A5: Field = Field(32);
pub const B5: Field = Field(33);
pub const C5: Field = Field(34);
pub const D5: Field = Field(35);
pub const E5: Field = Field(36);
pub const F5: Field = Field(37);
pub const G5: Field = Field(38);
pub const H5: Field = Field(39);

pub const A6: Field = Field(40);
pub const B6: Field = Field(41);
pub const C6: Field = Field(42);
pub const D6: Field = Field(43);
pub const E6: Field = Field(44);
pub const F6: Field = Field(45);
pub const G6: Field = Field(46);
pub const H6: Field = Field(47);

pub const A7: Field = Field(48);
pub const B7: Field = Field(49);
pub const C7: Field = Field(50);
pub const D7: Field = Field(51);
pub const E7: Field = Field(52);
pub const F7: Field = Field(53);
pub const G7: Field = Field(54);
pub const H7: Field = Field(55);

pub const A8: Field = Field(56);
pub const B8: Field = Field(57);
pub const C8: Field = Field(58);
pub const D8: Field = Field(59);
pub const E8: Field = Field(60);
pub const F8: Field = Field(61);
pub const G8: Field = Field(62);
pub const H8: Field = Field(63);

pub const COUNT: usize = 64;

pub struct FieldIter(usize);

impl Iterator for FieldIter {
    type Item = Field;

    fn next(&mut self) -> Option<Field> {
        if self.0 >= COUNT {
            None
        } else {
            let res = Some(Field(self.0));
            self.0 += 1;
            res
        }
    }
}

pub fn fields() -> FieldIter {
    FieldIter(0)
}

#[rustfmt::skip]
const NAMES: [&'static str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];

pub fn algebraic(f: Field) -> &'static str {
    NAMES[f.0]
}

pub fn from_algebraic(s: &str) -> Option<Field> {
    NAMES.iter().position(|&x| x == s).map(|x| Field(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algebraic_notation() {
        assert_eq!(algebraic(A1), "a1");
        assert_eq!(algebraic(E6), "e6");
        assert_eq!(algebraic(C4), "c4");
        assert_eq!(algebraic(F3), "f3");
    }

    fn from_algebraic_notation() {
        assert_eq!(from_algebraic("a1"), Some(A1));
        assert_eq!(from_algebraic("e7"), Some(E7));
        assert_eq!(from_algebraic("c3"), Some(C3));
        assert_eq!(from_algebraic("b9"), None);
        assert_eq!(from_algebraic(""), None);
        assert_eq!(from_algebraic("a1x"), None);
    }
}
