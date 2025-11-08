#[cfg(feature = "benchmark")]
use std::time::Instant;
use std::{fs, string::ToString};

use colored::Colorize;

use super::aoc_day::Aoc25Day;
use crate::solution::aoc_result::Aoc25Result;
use crate::time;

/// Trait containing all the boilerplate for a solution.
///
/// Method `parse_file_input` must be called before attempting
/// to solve the puzzles.
pub trait Aoc25Solution: Aoc25Day {
    // File path building

    fn build_input_path(&self) -> String {
        let day_nbr: usize = self.get_day_number();
        format!("data/inputs/day{day_nbr:0>2}.txt")
    }
    fn build_test_path(&self, test_extra: Option<String>) -> String {
        let day_nbr: usize = self.get_day_number();
        let extra: String = match test_extra {
            None => String::from(""),
            Some(val) => format!("-{val}"),
        };
        format!("data/tests/day{day_nbr:0>2}{extra}.txt")
    }

    fn get_file_data(&self, is_test: bool, test_extra: Option<String>) -> String {
        let path: String = match is_test {
            false => self.build_input_path(),
            true => self.build_test_path(test_extra),
        };
        fs::read_to_string(&path).unwrap_or_else(|_| panic!("Unable to read {}", &path))
    }

    // -- Main methods --

    /// Read the file and parse its content.
    fn parse_file_input(&mut self, is_test: bool, test_extra: Option<String>) {
        let file_data: String = self.get_file_data(is_test, test_extra);
        self.parse_input(file_data);
    }

    /// Parse the puzzle input.
    fn parse_input(&mut self, input: String);

    /// Implementation of the solution for part 1 of the puzzle
    fn solve_part_one(&mut self) -> Option<String> {
        None
    }

    /// Implementation of the solution for part 2 of the puzzle
    fn solve_part_two(&mut self) -> Option<String> {
        None
    }

    // -- Run method --

    #[cfg(not(feature = "benchmark"))]
    #[allow(unused_assignments)]
    fn run(&mut self, is_test: bool, test_extra: Option<String>) -> Aoc25Result {
        let mut part_one_result: Option<String> = None;
        let mut part_two_result: Option<String> = None;

        self.parse_file_input(is_test, test_extra);

        part_one_result = self.solve_part_one();

        if part_one_result.is_some() {
            part_two_result = self.solve_part_two();
        }

        Aoc25Result {
            day: self.get_day_number(),
            part_one_result,
            part_two_result,
        }
    }

    #[cfg(feature = "benchmark")]
    #[allow(unused_assignments)]
    fn run(&mut self, is_test: bool, test_extra: Option<String>) -> Aoc25Result {
        let mut part_one_result: Option<String> = None;
        let mut part_two_result: Option<String> = None;

        let mut parsing_duration: u128 = 0;
        let mut part_one_duration: u128 = 0;
        let mut part_two_duration: u128 = 0;

        //
        // Parse
        //

        // Exclude reading the file from the parsing duration
        let puzzle_input: String = self.get_file_data(is_test, test_extra);

        (_, parsing_duration) = time!(self.parse_input(puzzle_input));

        //
        // Part 1
        //

        (part_one_result, part_one_duration) = time!(self.solve_part_one());

        //
        // Part 2
        //

        if part_one_result.is_some() {
            (part_two_result, part_two_duration) = time!(self.solve_part_two());
        }

        Aoc25Result {
            day: self.get_day_number(),
            part_one_result,
            part_two_result,
            parsing_duration,
            part_one_duration,
            part_two_duration,
        }
    }
}
