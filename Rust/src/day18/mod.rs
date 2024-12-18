mod dijkstra;
mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day18::model::Tile18;
use crate::day18::parse::parse_input;
use crate::day18::solve::{solve_part_one, solve_part_two};
use crate::toolbox::{Coordinates, Grid};

const AXIS_LENGTH: usize = 71;
const TEST_AXIS_LENGTH: usize = 7;

#[derive(Default)]
pub struct Day18 {
    falling_bytes_coordinates: Vec<Coordinates>,
}

impl Aoc24Solution for Day18 {
    fn get_day_number(&self) -> usize {
        18
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        let length: usize = if is_test {
            TEST_AXIS_LENGTH
        } else {
            AXIS_LENGTH
        };
        self.falling_bytes_coordinates = parse_input(&self.get_data(is_test));

        solve_part_one(&self.falling_bytes_coordinates, length)
    }

    fn solve_part_two(&mut self, is_test: bool) -> i128 {
        let length: usize = if is_test {
            TEST_AXIS_LENGTH
        } else {
            AXIS_LENGTH
        };
        solve_part_two(&self.falling_bytes_coordinates, length)
    }
}
