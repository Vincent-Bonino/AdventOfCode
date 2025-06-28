mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day14::model::SecurityRobot;
use crate::day14::parse::parser_input;
use crate::day14::solve::{solve_part_one, solve_part_two};

#[derive(Default)]
pub struct Day14 {
    security_robots: Vec<SecurityRobot>,
}

impl Aoc24Solution for Day14 {
    fn get_day_number(&self) -> usize {
        14
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.security_robots = parser_input(&self.get_data(is_test));
        solve_part_one(&self.security_robots) as i128
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.security_robots) as i128
    }
}
