use aoc25_macros::Aoc25Day;

use super::logic::{solve_part_one, solve_part_two};
use super::model::Problem;
use super::parsing::parse_input;
use crate::prelude::*;

#[derive(Default, Aoc25Day)]
/// Solution for day 06.
pub struct Day06 {
    raw_numbers_lines: Vec<String>,
    operators: Vec<char>,
}

impl Aoc25Solution for Day06 {
    fn parse_input(&mut self, input: String) {
        (self.raw_numbers_lines, self.operators) = parse_input(&input);
    }

    fn solve_part_one(&mut self) -> Option<String> {
        Some(solve_part_one(&self.raw_numbers_lines, &self.operators))
    }

    fn solve_part_two(&mut self) -> Option<String> {
        Some(solve_part_two(&self.raw_numbers_lines, &self.operators))
    }
}
