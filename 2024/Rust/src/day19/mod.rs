mod parse;
mod solve;

use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

use crate::aoc::Aoc24Solution;
use crate::day19::parse::parse_input;
use crate::day19::solve::count_possibilities;

#[derive(Default)]
pub struct Day19 {
    towels: HashSet<String>,
    patterns: Vec<String>,

    // Special case where both results are computed at the same time
    part_one_result: i128,
    part_two_result: i128,
}

impl Aoc24Solution for Day19 {
    fn get_day_number(&self) -> usize {
        19
    }

    fn solve_part_one(&mut self, is_test: bool) -> i128 {
        self.solve(is_test);
        self.part_one_result
    }

    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        self.part_two_result
    }
}

impl Day19 {
    fn solve(&mut self, is_test: bool) {
        (self.towels, self.patterns) = parse_input(&self.get_data(is_test));

        let min_len: usize = self.towels.iter().map(|tow| tow.len()).min().unwrap();
        let max_len: usize = self.towels.iter().map(|tow| tow.len()).max().unwrap();

        let cache: HashMap<&str, i64> = HashMap::new();
        let counts: Vec<i64> = self
            .patterns
            .par_iter()
            .map(|pat| count_possibilities(&self.towels, pat, &mut cache.clone(), min_len, max_len))
            .collect();

        self.part_one_result = counts.iter().filter(|x| **x != 0).count() as i128;
        self.part_two_result = counts.iter().sum::<i64>() as i128;
    }
}
