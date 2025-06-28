mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day05::parse::parse_input;
use crate::day05::solve::{solve_part_one, solve_part_two};

#[derive(Default)]
pub struct Day05 {
    rules: Vec<(i32, i32)>,
    updates: Vec<Vec<i32>>,
}

impl Aoc24Solution for Day05 {
    fn get_day_number(&self) -> usize {
        5
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        (self.rules, self.updates) = parse_input(&self.get_data(is_test));
        solve_part_one(&self.rules, &self.updates)
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.rules, &mut self.updates)
    }
}
