mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day13::model::ClawMachine;
use crate::day13::parse::parse_input;
use crate::day13::solve::{solve_part_one, solve_part_two};
use crate::toolbox::maths::extended_euclidean_algorithm;

#[derive(Default)]
pub struct Day13 {
    claw_machines: Vec<ClawMachine>,
}

impl Aoc24Solution for Day13 {
    fn get_day_number(&self) -> usize {
        13
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.claw_machines = parse_input(&self.get_data(is_test));
        solve_part_one(&self.claw_machines) as i128
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.claw_machines) as i128
    }
}
