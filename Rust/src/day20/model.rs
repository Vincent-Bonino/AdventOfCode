use std::fmt::{Debug, Formatter};

#[derive(Clone, Default, Eq, PartialEq)]
pub enum Tile20 {
    #[default]
    Empty,
    Wall,
}

impl Debug for Tile20 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile20::Empty => write!(f, "."),
            Tile20::Wall => write!(f, "#"),
        }
    }
}
