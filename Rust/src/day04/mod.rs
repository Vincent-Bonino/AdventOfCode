mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day04::model::Letter;
use crate::day04::parse::parse_input;
use crate::day04::solve::{solve_part_one, solve_part_two};
use crate::toolbox::Grid;

#[derive(Default)]
pub struct Day04 {
    grid: Grid<Letter>,
}

impl Aoc24Solution for Day04 {
    fn get_day_number(&self) -> usize {
        4
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.grid = parse_input(&self.get_data(is_test));
        solve_part_one(&self.grid)
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.grid)
    }
}
