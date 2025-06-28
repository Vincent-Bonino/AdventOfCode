mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day07::model::{Equation, Operator};
use crate::day07::parse::parse_input;
use crate::day07::solve::{solve_part_one, solve_part_two};

#[derive(Default)]
pub struct Day07 {
    equations: Vec<Equation>,
}

impl Aoc24Solution for Day07 {
    fn get_day_number(&self) -> usize {
        7
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.equations = parse_input(&self.get_data(is_test));
        solve_part_one(&self.equations)
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.equations)
    }
}
