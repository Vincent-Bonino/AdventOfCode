mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day03::model::Instruction;
use crate::day03::parse::parse_input;
use crate::day03::solve::{solve_part_one, solve_part_two};

#[derive(Default)]
pub struct Day03 {
    instructions: Vec<Instruction>,
}

impl Aoc24Solution for Day03 {
    fn get_day_number(&self) -> usize {
        3
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.instructions = parse_input(&self.get_data(is_test));
        solve_part_one(&self.instructions)
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.instructions)
    }
}
