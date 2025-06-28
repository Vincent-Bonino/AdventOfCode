use crate::toolbox::{CollectionHashMap, Coordinates, Grid};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Default, Eq, Hash, PartialEq)]
pub enum Tile08 {
    Antenna(char),
    #[default]
    Empty,
}

impl Debug for Tile08 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile08::Empty => write!(f, "."),
            Tile08::Antenna(frequency) => write!(f, "{}", frequency),
        }
    }
}

impl Grid<Tile08> {
    pub fn get_antennas_positions(&self) -> CollectionHashMap<char, Coordinates> {
        let mut result: CollectionHashMap<char, Coordinates> = CollectionHashMap::new();

        for i in 0..self.depth() {
            for j in 0..self.width() {
                let coord: Coordinates = Coordinates {
                    x: i as i32,
                    y: j as i32,
                };
                if let Tile08::Antenna(frequency) = self.get_ref(&coord).unwrap() {
                    result.insert(*frequency, coord)
                }
            }
        }

        result
    }
}
