mod model;
mod solve;

use crate::aoc::Aoc24Solution;
use crate::day09::model::Memory;
use crate::day09::solve::solve_part_two;

#[derive(Default)]
pub struct Day09 {
    memory: Memory,
}

impl Aoc24Solution for Day09 {
    fn get_day_number(&self) -> usize {
        9
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.memory = Memory::from_string(&self.get_data(is_test));
        self.memory.optimize_space().checksum() as i128
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.memory)
    }
}
