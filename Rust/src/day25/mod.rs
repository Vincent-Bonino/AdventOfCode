mod model;
mod parse;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day25::model::{Key, Lock};
use crate::day25::parse::parse_input;
use crate::day25::solve::solve_part_one;

#[derive(Default)]
pub struct Day25 {
    keys: Vec<Key>,
    locks: Vec<Lock>,
}

impl Aoc24Solution for Day25 {
    fn get_day_number(&self) -> usize {
        25
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        (self.keys, self.locks) = parse_input(&self.get_data(is_test));
        solve_part_one(&self.keys, &self.locks) as i128
    }
}
