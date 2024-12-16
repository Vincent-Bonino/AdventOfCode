mod dijkstra;
mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day16::dijkstra::custom_dijkstra;
use crate::day16::model::Tile16;
use crate::day16::parse::parse_input;
use crate::toolbox::{Coordinates, Grid};

#[derive(Default)]
pub struct Day16 {
    maze: Grid<Tile16>,
    start_coordinates: Coordinates,
    exit_coordinates: Coordinates,

    // Special case where both results are computed at the same time
    part_one_result: i64,
    part_two_result: i64,
}

impl Aoc24Solution for Day16 {
    fn get_day_number(&self) -> usize {
        16
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        (self.maze, self.start_coordinates, self.exit_coordinates) =
            parse_input(&self.get_data(is_test));
        (self.part_one_result, self.part_two_result) =
            custom_dijkstra(&self.maze, &self.start_coordinates, &self.exit_coordinates);

        self.part_one_result as i128
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        self.part_two_result as i128
    }
}
