use std::fmt::{Debug, Formatter};

#[derive(Clone, Default, Eq, PartialEq)]
pub enum Tile18 {
    #[default]
    Empty,
    Corrupted,
}

impl Debug for Tile18 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile18::Empty => write!(f, "."),
            Tile18::Corrupted => write!(f, "#"),
        }
    }
}
