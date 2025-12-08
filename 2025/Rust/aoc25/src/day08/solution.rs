use aoc25_macros::Aoc25Day;

use super::logic::{compute_distances, solve_part_one, solve_part_two};
use super::model::Point;
use super::parsing::parse_input;
use crate::prelude::*;

#[derive(Default, Aoc25Day)]
/// Solution for day 08.
pub struct Day08 {
    points: Vec<Point>,
    distances: Vec<(usize, usize, f32)>,
}

impl Aoc25Solution for Day08 {
    fn parse_input(&mut self, input: String) {
        self.points = parse_input(&input);
        self.distances = compute_distances(&self.points);
    }

    fn solve_part_one(&mut self) -> Option<String> {
        Some(solve_part_one(&self.distances))
    }

    fn solve_part_two(&mut self) -> Option<String> {
        Some(solve_part_two(&self.points, &self.distances))
    }
}
