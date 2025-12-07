use aoc25_macros::Aoc25Day;
use std::collections::HashMap;

use super::logic::{solve_part_one, solve_part_two};
use super::model::Tile07;
use crate::prelude::*;

#[derive(Default, Aoc25Day)]
/// Solution for day 07.
pub struct Day07 {
    tachyon_manifold: Grid<Tile07>,
    beam_coordinates: Coordinates,
}

impl Aoc25Solution for Day07 {
    fn parse_input(&mut self, input: String) {
        let (grid, positions): (Grid<Tile07>, HashMap<char, Coordinates>) =
            Grid::from_str_with_markers(&input, &['S']).unwrap();

        self.tachyon_manifold = grid;
        self.beam_coordinates = positions[&'S'];
    }

    fn solve_part_one(&mut self) -> Option<String> {
        Some(solve_part_one(&self.tachyon_manifold, &self.beam_coordinates))
    }

    fn solve_part_two(&mut self) -> Option<String> {
        Some(solve_part_two(&self.tachyon_manifold, &self.beam_coordinates))
    }
}
