use hashbrown::HashMap;
use std::fmt::Debug;

use crate::prelude::Direction;

pub struct Present {
    pub index: usize,
    pub shape: HashMap<usize, char>,
}

impl Present {
    /// This solution assumes that all presents have a 3x3 shape.
    ///
    /// This is verified during the parsing.
    pub const SIZE: usize = 3;

    pub const TOTAL_AREA: usize = Self::SIZE * Self::SIZE;

    pub fn actual_area(&self) -> usize {
        self.shape.iter().filter(|(_, v)| **v == '#').count()
    }
}

impl Debug for Present {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut shape: String = String::with_capacity(12);
        for index in 0..9 {
            shape.push(self.shape[&index]);
            if index % 3 == 2 {
                shape.push('\n');
            }
        }

        f.debug_struct("Present")
            .field("index", &self.index)
            .field("shape", &shape)
            .finish()
    }
}

#[derive(Debug)]
pub struct TreeRegion {
    pub height: usize,
    pub width: usize,

    pub present_quantity: Vec<usize>,
}

impl TreeRegion {
    pub fn area(&self) -> usize {
        self.height * self.width
    }

    pub fn total_quantity(&self) -> usize {
        self.present_quantity.iter().sum()
    }
}
