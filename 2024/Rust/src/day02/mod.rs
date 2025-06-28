mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day02::parse::parse_input;
use crate::day02::solve::{solve_part_one, solve_part_two};

#[derive(Default)]
pub struct Day02 {
    reports: Vec<Vec<i32>>,
}

impl Aoc24Solution for Day02 {
    fn get_day_number(&self) -> usize {
        2
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.reports = parse_input(&self.get_data(is_test));
        solve_part_one(&self.reports)
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.reports)
    }
}
