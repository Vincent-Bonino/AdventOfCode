use crate::day12::model::Region;
use crate::toolbox::{Coordinates, Grid};
use std::collections::HashSet;

pub fn solve_part_one(garden: &Grid<char>) -> i32 {
    build_regions(garden)
        .iter()
        .map(|region| region.area() * region.perimeter())
        .sum()
}

pub fn solve_part_two(garden: &Grid<char>) -> i32 {
    build_regions(garden)
        .iter()
        .map(|region| region.area() * region.side_number())
        .sum()
}

pub fn build_regions(garden: &Grid<char>) -> Vec<Region> {
    // Store already counted tiles, to prevent double counts
    let mut counted: HashSet<Coordinates> = HashSet::with_capacity(garden.depth() * garden.width());
    let mut regions: Vec<Region> = Vec::new();

    // Starting at each position of the garden, compute a region
    for (coord, elmt) in garden.enumerate() {
        // No new region if it has already been counted
        if counted.contains(&coord) {
            continue;
        }

        let mut current_region: Region = Region::new(*elmt);
        let mut to_explore: Vec<Coordinates> = Vec::from([coord]);

        while let Some(current_position) = to_explore.pop() {
            if counted.contains(&current_position) {
                continue;
            }

            // Do not count coordinates with another plant type
            if garden.get_ref(&current_position) != Some(&current_region.plant_type) {
                continue;
            }

            // Current position can be added to the current region
            // Its neighbours are added to the positions to explore
            current_region.add_coordinates(current_position);
            counted.insert(current_position);
            to_explore.extend(current_position.neighbours4())
        }

        regions.push(current_region);
    }

    regions
}
