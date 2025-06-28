use std::collections::HashMap;

use itertools::Itertools;
use rayon::prelude::*;

use crate::day06::model::Tile06;
use crate::toolbox::{Coordinates, Direction, Grid};

pub fn solve_part_one(grid: &Grid<Tile06>, starting_position: &Coordinates) -> i128 {
    let mut walked_tiles: Vec<Coordinates> = Vec::new();

    let mut current_direction: Direction = Direction::N;
    let mut current_position: Coordinates = *starting_position;

    // Loop while the guard in within the provided map
    loop {
        let next_position: Coordinates = current_position.step(&current_direction);
        match grid.get_ref(&next_position) {
            None => {
                walked_tiles.push(current_position);
                break; // Out of bounds
            }
            Some(Tile06::Blocked) => current_direction = current_direction.right(),
            Some(Tile06::Free) => {
                walked_tiles.push(current_position);
                current_position = next_position;
            }
        }
    }

    // Removing tiles visited more than once
    let unique_walked_tiles: Vec<&Coordinates> = walked_tiles.iter().unique().collect();
    unique_walked_tiles.len() as i128
}

fn has_loop(grid: &Grid<Tile06>, starting_position: &Coordinates) -> bool {
    // Keep track of all walked tiles (and their direction)
    let mut cache: HashMap<(Coordinates, Direction), Option<i32>> = HashMap::new();

    let mut current_direction: Direction = Direction::N;
    let mut current_position: Coordinates = *starting_position;

    // Walk the guard until they walk out of the map or start a loop
    loop {
        let next_position: Coordinates = current_position.step(&current_direction);

        // Test if a loop is starting
        if cache.contains_key(&(next_position, current_direction)) {
            return true;
        }

        match grid.get_ref(&next_position) {
            None => return false, // No loop, the guard walked away
            Some(Tile06::Blocked) => current_direction = current_direction.right(),
            Some(Tile06::Free) => {
                cache.insert((next_position, current_direction), None);
                current_position = next_position;
            }
        }
    }
}

// Terrible solution, consisting of trying all different maps.
// Take only 2 secs on my computer with release build
pub fn solve_part_two(grid: &Grid<Tile06>, starting_position: &Coordinates) -> i128 {
    let mut indexes: Vec<(usize, usize)> = Vec::with_capacity(grid.depth() * grid.width());

    for i in 0..grid.depth() {
        for j in 0..grid.width() {
            indexes.push((i, j));
        }
    }

    indexes
        .into_par_iter()
        .map(|(i, j)| {
            let position: Coordinates = Coordinates {
                x: i as i32,
                y: j as i32,
            };

            // Skip if target is already a blocked tile
            if let Some(Tile06::Blocked) = grid.get_ref(&position) {
                return 0;
            }

            // Skip if target is the starting position
            if position == *starting_position {
                return 0;
            }

            // Build the new grid and look for a loop
            let mut tmp_grid: Grid<Tile06> = (*grid).clone();
            tmp_grid.replace(&position, Tile06::Blocked);

            if has_loop(&tmp_grid, starting_position) {
                1
            } else {
                0
            }
        })
        .sum::<i128>()
}
