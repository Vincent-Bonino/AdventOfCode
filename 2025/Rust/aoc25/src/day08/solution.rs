use aoc25_macros::Aoc25Day;

use super::logic::{compute_distances, solve_both_parts};
use super::model::Point;
use super::parsing::parse_input;
use crate::prelude::*;

#[derive(Default, Aoc25Day)]
/// Solution for day 08.
pub struct Day08 {
    points: Vec<Point>,
    distances: Vec<(usize, usize, f32)>,

    part1: String,
    part2: String,
}

impl Aoc25Solution for Day08 {
    fn parse_input(&mut self, input: String) {
        self.points = parse_input(&input);
        self.distances = compute_distances(&self.points);
    }

    fn solve_part_one(&mut self) -> Option<String> {
        (self.part1, self.part2) = solve_both_parts(&self.points, &self.distances);

        Some(self.part1.clone())
    }

    fn solve_part_two(&mut self) -> Option<String> {
        // Allow running part two only
        if self.part1 == String::default() {
            (self.part1, self.part2) = solve_both_parts(&self.points, &self.distances)
        }
        Some(self.part2.clone())
    }
}
