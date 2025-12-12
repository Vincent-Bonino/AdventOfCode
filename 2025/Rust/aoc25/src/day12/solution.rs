use super::model::{Present, TreeRegion};
use crate::day12::logic::solve_part_one;
use crate::day12::parsing::parse_input;
use crate::prelude::*;
use aoc25_macros::Aoc25Day;

#[derive(Default, Aoc25Day)]
/// Solution for day 12.
pub struct Day12 {
    presents: Vec<Present>,
    regions: Vec<TreeRegion>,
}

impl Aoc25Solution for Day12 {
    fn parse_input(&mut self, input: String) {
        let (presents, regions) = parse_input(&input);
        self.presents = presents;
        self.regions = regions;
    }

    fn solve_part_one(&mut self) -> Option<String> {
        Some(solve_part_one(&self.presents, &self.regions))
    }
}
