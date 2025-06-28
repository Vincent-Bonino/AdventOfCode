pub mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day22::parse::parse_input;
use crate::day22::solve::{solve_part_one, solve_part_two};

#[derive(Default)]
pub struct Day22 {
    seeds: Vec<u64>,
}

impl Aoc24Solution for Day22 {
    fn get_day_number(&self) -> usize {
        22
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.seeds = parse_input(&self.get_data(is_test));
        solve_part_one(&self.seeds) as i128
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.seeds) as i128
    }
}
