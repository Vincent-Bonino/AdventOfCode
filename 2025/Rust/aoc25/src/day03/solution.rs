use aoc25_macros::Aoc25Day;

use super::logic::solve;
use crate::prelude::*;

const PART_ONE_BATTERY_COUNT: usize = 2;
const PART_TWO_BATTERY_COUNT: usize = 12;

#[derive(Default, Aoc25Day)]
/// Solution for day 03.
pub struct Day03 {
    banks: Vec<Vec<char>>,
}

impl Aoc25Solution for Day03 {
    fn parse_input(&mut self, input: String) {
        self.banks = input.lines().map(|l| l.chars().collect()).collect();
    }

    fn solve_part_one(&mut self) -> Option<String> {
        Some(solve(&self.banks, PART_ONE_BATTERY_COUNT))
    }

    fn solve_part_two(&mut self) -> Option<String> {
        Some(solve(&self.banks, PART_TWO_BATTERY_COUNT))
    }
}
