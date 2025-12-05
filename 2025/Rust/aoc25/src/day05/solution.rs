use super::model::FreshRange;
use super::parsing::parse_input;
use crate::day05::logic::{merge_ranges, solve_part_one, solve_part_two};
use crate::prelude::*;
use aoc25_macros::Aoc25Day;

#[derive(Default, Aoc25Day)]
/// Solution for day 05.
pub struct Day05 {
    fresh_ranges: Vec<FreshRange>,
    ids: Vec<u64>,

    merged: bool,
}

impl Day05 {
    fn prepare(&mut self) {
        if !self.merged {
            self.fresh_ranges = merge_ranges(&self.fresh_ranges);
            self.merged = true;
        }
    }
}

impl Aoc25Solution for Day05 {
    fn parse_input(&mut self, input: String) {
        (self.fresh_ranges, self.ids) = parse_input(&input);
    }

    fn solve_part_one(&mut self) -> Option<String> {
        self.prepare();
        Some(solve_part_one(&self.fresh_ranges, &self.ids))
    }

    fn solve_part_two(&mut self) -> Option<String> {
        self.prepare();
        Some(solve_part_two(&self.fresh_ranges))
    }
}
