mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day06::model::Tile06;
use crate::day06::parse::parse_input;
use crate::day06::solve::{solve_part_one, solve_part_two};
use crate::toolbox::{Coordinates, Grid};

#[derive(Default)]
pub struct Day06 {
    grid: Grid<Tile06>,
    starting_position: Coordinates,
}

impl Aoc24Solution for Day06 {
    fn get_day_number(&self) -> usize {
        6
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        (self.grid, self.starting_position) = parse_input(&self.get_data(is_test));
        solve_part_one(&self.grid, &self.starting_position)
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.grid, &self.starting_position)
    }
}
