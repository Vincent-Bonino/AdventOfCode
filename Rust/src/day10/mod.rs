mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day10::parse::parse_input;
use crate::day10::solve::{solve_part_one, solve_part_two};
use crate::toolbox::Grid;

#[derive(Default)]
pub struct Day10 {
    topographic_map: Grid<u32>,
}

impl Aoc24Solution for Day10 {
    fn get_day_number(&self) -> usize {
        10
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.topographic_map = parse_input(&self.get_data(is_test));
        solve_part_one(&self.topographic_map)
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.topographic_map)
    }
}
