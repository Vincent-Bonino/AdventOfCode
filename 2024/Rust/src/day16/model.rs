use std::fmt::{Debug, Formatter};

#[derive(Default, Eq, PartialEq)]
pub enum Tile16 {
    #[default]
    Empty,
    Wall,
}

impl Debug for Tile16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile16::Empty => write!(f, "."),
            Tile16::Wall => write!(f, "#"),
        }
    }
}
