use aoc25_macros::Aoc25Day;

use super::logic::{solve_part_one, solve_part_two};
use super::parsing::parse_input;
use crate::prelude::*;

#[derive(Default, Aoc25Day)]
/// Solution for day 09.
pub struct Day09 {
    red_tiles: Vec<Coordinates>,
}

impl Aoc25Solution for Day09 {
    fn parse_input(&mut self, input: String) {
        self.red_tiles = parse_input(&input);
    }

    fn solve_part_one(&mut self) -> Option<String> {
        Some(solve_part_one(&self.red_tiles))
    }

    fn solve_part_two(&mut self) -> Option<String> {
        Some(solve_part_two(&self.red_tiles))
    }
}
