use aoc25_macros::Aoc25Day;
use hashbrown::HashMap;

use super::logic::{solve_part_one, solve_part_two};
use super::parsing::parse_input;
use crate::prelude::*;

#[derive(Default, Aoc25Day)]
/// Solution for day 02.
pub struct Day02 {
    ranges: Vec<(u64, u64)>,
}

impl Aoc25Solution for Day02 {
    fn parse_input(&mut self, input: String) {
        self.ranges = parse_input(&input)
    }

    fn solve_part_one(&mut self) -> Option<String> {
        Some(solve_part_one(&self.ranges))
    }

    fn solve_part_two(&mut self) -> Option<String> {
        Some(solve_part_two(&self.ranges))
    }
}
