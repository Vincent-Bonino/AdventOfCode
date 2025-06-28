use itertools::Itertools;
use rayon::prelude::*;

use crate::day08::model::Tile08;
use crate::toolbox::{CollectionHashMap, Coordinates, Grid};

pub fn solve_part_one(map: &Grid<Tile08>) -> i128 {
    let antennas: CollectionHashMap<char, Coordinates> = map.get_antennas_positions();

    let result: usize = antennas
        .hash_map
        .iter()
        .flat_map(|(_key, value)| {
            let mut result: Vec<Coordinates> = Vec::with_capacity(2);

            for combination in value.iter().combinations(2) {
                let coord1: &Coordinates = combination.first().unwrap();
                let coord2: &Coordinates = combination.get(1).unwrap();

                let delta: (i32, i32) = coord2.get_delta(coord1);

                let coord_before = coord1.with_delta(delta, false);
                let coord_after = coord2.with_delta(delta, true);

                if map.is_in_bounds(&coord_before) {
                    result.push(coord_before)
                }
                if map.is_in_bounds(&coord_after) {
                    result.push(coord_after)
                }
            }
            result
        })
        .unique()
        .count();

    result as i128
}

pub fn solve_part_two(map: &Grid<Tile08>) -> i128 {
    let antennas: CollectionHashMap<char, Coordinates> = map.get_antennas_positions();

    let result: usize = antennas
        .hash_map
        .iter()
        .flat_map(|(_key, value)| {
            let mut result: Vec<Coordinates> = Vec::with_capacity(2);

            for combination in value.iter().combinations(2) {
                let coord1: &Coordinates = combination.first().unwrap();
                let coord2: &Coordinates = combination.get(1).unwrap();
                let delta: (i32, i32) = coord2.get_delta(coord1);

                // If at least two antenna exist (which is the case here)
                // an anti-node appears on each of them
                result.push(*coord1);
                result.push(*coord2);

                let mut next_coord: Coordinates;

                // Before
                next_coord = coord1.with_delta(delta, false);
                while map.is_in_bounds(&next_coord) {
                    result.push(next_coord);
                    next_coord = next_coord.with_delta(delta, false);
                }

                // After
                next_coord = coord2.with_delta(delta, true);
                while map.is_in_bounds(&next_coord) {
                    result.push(next_coord);
                    next_coord = next_coord.with_delta(delta, true);
                }
            }
            result
        })
        .unique()
        .count();

    result as i128
}
