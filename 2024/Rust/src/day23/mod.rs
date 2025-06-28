mod parse;
mod solve;

use hashbrown::HashMap;

use crate::aoc::Aoc24Solution;
use crate::day23::parse::parse_input;
use crate::day23::solve::{solve_part_one, solve_part_two};

#[derive(Default)]
pub struct Day23 {
    graph: HashMap<String, Vec<String>>,
}

impl Aoc24Solution for Day23 {
    fn get_day_number(&self) -> usize {
        23
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.graph = parse_input(&self.get_data(is_test));
        solve_part_one(&self.graph) as i128
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        solve_part_two(&self.graph);
        123 // Dummy value
    }
}
