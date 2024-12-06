use crate::toolbox::Grid;
use std::fmt::{Debug, Formatter};

#[derive(Clone, Default)]
pub enum Tile06 {
    #[default]
    Free,
    Blocked,
}

impl Debug for Tile06 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile06::Free => write!(f, "."),
            Tile06::Blocked => write!(f, "#"),
        }
    }
}
