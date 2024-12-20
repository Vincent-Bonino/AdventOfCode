mod compute;
mod dijkstra;
mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day20::model::Tile20;
use crate::day20::parse::parse_input;
use crate::day20::solve::{solve_part_one, solve_part_two};
use crate::toolbox::{Coordinates, Grid};

#[derive(Default)]
pub struct Day20 {
    racetrack: Grid<Tile20>,
    start: Coordinates,
    end: Coordinates,
}

impl Aoc24Solution for Day20 {
    fn get_day_number(&self) -> usize {
        20
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        (self.racetrack, self.start, self.end) = parse_input(&self.get_data(is_test));
        solve_part_one(&self.racetrack, &self.start, &self.end) as i128
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.racetrack, &self.start, &self.end) as i128
    }
}
