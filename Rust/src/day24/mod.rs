mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day24::model::CableCircuit;
use crate::day24::parse::parse_input;
use crate::day24::solve::{solve_part_one, solve_part_two};

#[derive(Default)]
pub struct Day24 {
    cable_circuit: CableCircuit,
}

impl Aoc24Solution for Day24 {
    fn get_day_number(&self) -> usize {
        24
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.cable_circuit = parse_input(&self.get_data(is_test));
        solve_part_one(&self.cable_circuit) as i128
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.cable_circuit) as i128
    }
}
