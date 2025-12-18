use aoc25_macros::Aoc25Day;

use super::logic::{solve_part_one, solve_part_two};
use super::model::InputLine;
use super::parsing::parse_input;
use crate::prelude::*;

#[derive(Default, Aoc25Day)]
/// Solution for day 10.
pub struct Day10 {
    data: Vec<InputLine>,
}

impl Aoc25Solution for Day10 {
    fn parse_input(&mut self, input: String) {
        self.data = parse_input(&input);
    }

    fn solve_part_one(&mut self) -> Option<String> {
        Some(solve_part_one(&self.data))
    }

    fn solve_part_two(&mut self) -> Option<String> {
        Some(solve_part_two(&self.data))
    }
}
