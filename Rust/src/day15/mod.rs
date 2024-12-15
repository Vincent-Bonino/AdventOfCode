mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day15::model::Tile15;
use crate::day15::parse::parse_input;
use crate::day15::solve::{solve_part_one, solve_part_two};
use crate::toolbox::{Coordinates, Direction, Grid};

#[derive(Default)]
pub struct Day15 {
    warehouse: Grid<Tile15>,
    instructions: Vec<Direction>,
    robot_position: Coordinates,
}

impl Aoc24Solution for Day15 {
    fn get_day_number(&self) -> usize {
        15
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        (self.warehouse, self.robot_position, self.instructions) =
            parse_input(&self.get_data(is_test));
        solve_part_one(&self.warehouse, &self.robot_position, &self.instructions) as i128
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.warehouse, &self.robot_position, &self.instructions) as i128
    }
}
