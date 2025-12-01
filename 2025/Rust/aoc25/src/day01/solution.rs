use aoc25_macros::Aoc25Day;

use super::logic::solve_both_parts;
use super::parsing::parse_input;
use crate::prelude::*;

#[derive(Default, Aoc25Day)]
/// Solution for day 01.
pub struct Day01 {
    rotations: Vec<i32>,

    part1: String,
    part2: String,
}

impl Aoc25Solution for Day01 {
    fn parse_input(&mut self, input: String) {
        self.rotations = parse_input(&input)
    }

    fn solve_part_one(&mut self) -> Option<String> {
        (self.part1, self.part2) = solve_both_parts(&self.rotations);
        Some(self.part1.clone())
    }

    fn solve_part_two(&mut self) -> Option<String> {
        // Allow running part two only
        if self.part1 == String::default() {
            (self.part1, self.part2) = solve_both_parts(&self.rotations);
        }

        Some(self.part2.clone())
    }
}
