mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day11::model::Stone;
use crate::day11::parse::parse_input;
use crate::day11::solve::{solve_part_one, solve_part_two};

#[derive(Default)]
pub struct Day11 {
    stones: Vec<Stone>,
}

impl Aoc24Solution for Day11 {
    fn get_day_number(&self) -> usize {
        11
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.stones = parse_input(&self.get_data(is_test));
        solve_part_one(&self.stones)
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.stones)
    }
}
