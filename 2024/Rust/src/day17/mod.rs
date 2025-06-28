mod model;
mod parse;
mod reversed;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day17::model::Computer;
use crate::day17::parse::parse_input;
use crate::day17::solve::{solve_part_one, solve_part_two};
use crate::toolbox::{Coordinates, Grid};

#[derive(Default)]
pub struct Day17 {
    computer: Computer,
}

impl Aoc24Solution for Day17 {
    fn get_day_number(&self) -> usize {
        17
    }

    fn get_extra_name(&self) -> Option<&str> {
        Some("2")
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.computer = parse_input(&self.get_data(is_test));
        solve_part_one(&self.computer)
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.computer)
    }
}
