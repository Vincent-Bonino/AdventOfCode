mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day01::parse::parse_input;
use crate::day01::solve::{solve_part_one, solve_part_two};

#[derive(Default)]
pub struct Day01 {
    left_list: Vec<i32>,
    right_list: Vec<i32>,
}

impl Aoc24Solution for Day01 {
    fn get_day_number(&self) -> usize {
        1
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        (self.left_list, self.right_list) = parse_input(&self.get_data(is_test));
        solve_part_one(&self.left_list, &self.right_list)
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.left_list, &self.right_list)
    }
}
