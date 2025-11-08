use aoc25_macros::Aoc25Day;

use super::model::Tile00;
use crate::prelude::*;

#[derive(Default, Aoc25Day)]
/// Example solution.
pub struct Day00 {
    grid: Grid<Tile00>,
    start: Coordinates,
    end: Coordinates,
}

impl Aoc25Solution for Day00 {
    fn parse_input(&mut self, input: String) {
        let (grid, markers) = Grid::from_str_with_markers(&input, &['E', 'S']).unwrap();
        self.grid = grid;
        self.start = *markers.get(&'S').expect("Start not found");
        self.end = *markers.get(&'E').expect("End not found");
    }

    fn solve_part_one(&mut self) -> Option<String> {
        Some(String::from("1234"))
    }

    // fn solve_part_two(&mut self) -> Option<String> {
    //     Some(solve_part_two(&self.grid, &self.start, &self.end))
    // }
}
