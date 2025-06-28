mod model;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day12::solve::{solve_part_one, solve_part_two};
use crate::toolbox::Grid;

#[derive(Default)]
pub struct Day12 {
    garden: Grid<char>,
}

impl Aoc24Solution for Day12 {
    fn get_day_number(&self) -> usize {
        12
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.garden = Grid::from_string(&self.get_data(is_test));
        solve_part_one(&self.garden) as i128
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.garden) as i128
    }
}
