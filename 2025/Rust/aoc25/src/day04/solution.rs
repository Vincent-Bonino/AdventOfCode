use std::str::FromStr;

use hashbrown::HashSet;

use aoc25_macros::Aoc25Day;

use super::logic::{solve_part_one, solve_part_two};
use super::model::Tile04;
use crate::prelude::*;

#[derive(Default, Aoc25Day)]
/// Solution for day 04.
pub struct Day04 {
    paper_rolls: HashSet<Coordinates>,
}

impl Aoc25Solution for Day04 {
    fn parse_input(&mut self, input: String) {
        self.paper_rolls = Grid::from_str(&input)
            .expect("Failed to parse input.")
            .into_iter()
            .coord_enumerate()
            .filter_map(|(coord, tile): (Coordinates, Tile04)| {
                if let Tile04::PaperRoll = tile {
                    Some(coord)
                } else {
                    None
                }
            })
            .collect();
    }

    fn solve_part_one(&mut self) -> Option<String> {
        Some(solve_part_one(&self.paper_rolls))
    }

    fn solve_part_two(&mut self) -> Option<String> {
        Some(solve_part_two(&self.paper_rolls))
    }
}
