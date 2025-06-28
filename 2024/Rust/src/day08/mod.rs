mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day08::model::Tile08;
use crate::day08::parse::parse_input;
use crate::day08::solve::{solve_part_one, solve_part_two};
use crate::toolbox::Grid;

#[derive(Default)]
pub struct Day08 {
    antenna_map: Grid<Tile08>,
}

impl Aoc24Solution for Day08 {
    fn get_day_number(&self) -> usize {
        8
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.antenna_map = parse_input(&self.get_data(is_test));
        solve_part_one(&self.antenna_map)
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.antenna_map)
    }
}
