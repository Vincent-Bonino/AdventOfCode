use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use crate::prelude::*;

pub fn solve_part_one(paper_rolls: &HashSet<Coordinates>) -> String {
    let total_rolls = paper_rolls.len();
    let neighbours_cache: HashMap<&Coordinates, Vec<Coordinates>> =
        paper_rolls.iter().map(|coord| (coord, coord.neighbours8())).collect();

    let rolls_after = remove_accessible_rolls(paper_rolls, &neighbours_cache).len();

    (total_rolls - rolls_after).to_string()
}

pub fn solve_part_two(paper_rolls: &HashSet<Coordinates>) -> String {
    let total_rolls = paper_rolls.len();
    let neighbours_cache: HashMap<&Coordinates, Vec<Coordinates>> =
        paper_rolls.iter().map(|coord| (coord, coord.neighbours8())).collect();

    let mut updated_rolls: HashSet<Coordinates> = paper_rolls.clone();
    loop {
        let next_rolls = remove_accessible_rolls(&updated_rolls, &neighbours_cache);

        if updated_rolls.len() == next_rolls.len() {
            break;
        } else {
            updated_rolls = next_rolls;
        }
    }

    let rolls_after = updated_rolls.len();
    (total_rolls - rolls_after).to_string()
}

pub fn remove_accessible_rolls(
    paper_rolls: &HashSet<Coordinates>,
    neighbours_cache: &HashMap<&Coordinates, Vec<Coordinates>>,
) -> HashSet<Coordinates> {
    paper_rolls
        .iter()
        .filter(|coord| {
            let neighbours_count: usize = neighbours_cache
                .get(coord)
                .unwrap()
                .iter()
                .filter(|neighbour| paper_rolls.contains(*neighbour))
                .count();
            neighbours_count >= 4
        })
        .cloned()
        .collect()
}
