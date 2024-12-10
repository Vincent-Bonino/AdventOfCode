use crate::toolbox::{Coordinates, Direction, Grid};
use itertools::Itertools;
use std::collections::HashMap;
use std::io::Stdin;

type Trail = Vec<Coordinates>;

const TARGET_HEIGHT: usize = 9;

pub fn solve_part_one(map: &Grid<u32>) -> i128 {
    map.get_coordinates_vec()
        .iter()
        .map(|coord| get_position_score(map, coord))
        .sum()
}

pub fn solve_part_two(map: &Grid<u32>) -> i128 {
    map.get_coordinates_vec()
        .iter()
        .map(|coord| get_position_rating(map, coord))
        .sum()
}

/// Final computation for part one
fn get_position_score(map: &Grid<u32>, starting_position: &Coordinates) -> i128 {
    let valid_trails: Vec<Trail> = get_trails(map, starting_position);

    valid_trails
        .iter()
        .map(|trail| trail.last())
        .unique()
        .count() as i128
}

/// Final computation for part two
fn get_position_rating(map: &Grid<u32>, starting_position: &Coordinates) -> i128 {
    let valid_trails: Vec<Trail> = get_trails(map, starting_position);

    valid_trails.len() as i128
}

/// Return all the valid trails starting at the given position
///
/// Weird implementation of a BFS
fn get_trails(map: &Grid<u32>, starting_position: &Coordinates) -> Vec<Trail> {
    if map.get_ref(starting_position) != Some(&0) {
        return Vec::new();
    }

    // Starting at the given position, look for trails

    let mut trails: HashMap<usize, Vec<Trail>> = HashMap::new();
    trails.insert(0, vec![vec![*starting_position]]);

    // Build paths reaching incremental heights
    for height in 0..TARGET_HEIGHT {
        let current_trails: Option<&Vec<Trail>> = trails.get(&height);
        if current_trails.is_none() {
            break;
        }

        let mut new_trails: Vec<Trail> = Vec::with_capacity(current_trails.unwrap().len());
        let target_height: u32 = (height + 1) as u32;

        for trail in current_trails.unwrap() {
            let current_position: &Coordinates = trail.last().expect("Empty trail");

            // Look at the four tiles around
            for next_direction in Direction::neighbours4() {
                let next_position: Coordinates = current_position.step(&next_direction);

                if Some(&target_height) == map.get_ref(&next_position) {
                    let mut new_trail: Trail = trail.clone();
                    new_trail.push(next_position);

                    new_trails.push(new_trail);
                }
            }
        }

        trails.insert(target_height as usize, new_trails);
    }

    trails.remove(&TARGET_HEIGHT).unwrap_or_default()
}
