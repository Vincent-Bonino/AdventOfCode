use std::collections::VecDeque;

use hashbrown::{HashMap, HashSet};

use super::model::Tile07;
use crate::prelude::{Coordinates, Direction, Grid};

/// Solve part 1.
///
/// Logic:
///  - Propagate beams, one at a time, to the bottom.
///  - Mark tiles it propagated to.
///  - Stop if encountering a tile that another beam already marked,
///    to prevent double counting a split.
pub fn solve_part_one(reference_manifold: &Grid<Tile07>, start_position: &Coordinates) -> String {
    let mut beams: VecDeque<Coordinates> = VecDeque::from(vec![*start_position]);
    let mut manifold: Grid<Tile07> = reference_manifold.clone();
    let mut split_count: usize = 0;

    while let Some(coords) = beams.pop_front() {
        let mut next_coords: Coordinates = coords;

        'propagate: loop {
            next_coords = next_coords.step(&Direction::S);

            match manifold.get_ref(&next_coords) {
                // Out of bounds
                None => break 'propagate,
                // Already a beam here, do not double count
                Some(Tile07::Beam) => break 'propagate,
                // Nothing to do
                Some(Tile07::Empty) => {
                    manifold.replace(&next_coords, Tile07::Beam);
                }
                // Split
                Some(Tile07::Splitter) => {
                    split_count += 1;

                    let left: Coordinates = next_coords.step(&Direction::W);
                    let right: Coordinates = next_coords.step(&Direction::E);
                    let to_the_left: Option<&Tile07> = manifold.get_ref(&left);
                    let to_the_right: Option<&Tile07> = manifold.get_ref(&right);

                    if let Some(Tile07::Empty) = to_the_left {
                        beams.push_back(left);
                    }
                    if let Some(Tile07::Empty) = to_the_right {
                        beams.push_back(right);
                    }

                    break 'propagate;
                }
            }
        }
    }

    split_count.to_string()
}

/// Solve part 2.
///
/// Logic:
///  - Propagate all beams one level down.
///  - Merge beams that would follow the same path,
///    adding their weight to account for multiple timelines while computing only once.
pub fn solve_part_two(manifold: &Grid<Tile07>, start_position: &Coordinates) -> String {
    let mut beams: HashMap<Coordinates, usize> = HashMap::with_capacity(manifold.cols);
    let mut timeline_count: usize = 0;

    beams.insert(*start_position, 1);

    while !beams.is_empty() {
        let mut new_beams: HashMap<Coordinates, usize> = HashMap::with_capacity(manifold.cols);

        for (beam_coord, beam_weight) in &beams {
            let next_coords: Coordinates = beam_coord.step(&Direction::S);

            match manifold.get_ref(&next_coords) {
                // Ouf of bound, count the gathered timelines
                None => timeline_count += beam_weight,
                // Split
                Some(Tile07::Splitter) => {
                    let left: Coordinates = next_coords.step(&Direction::W);
                    let right: Coordinates = next_coords.step(&Direction::E);

                    new_beams
                        .entry(left)
                        .and_modify(|e| *e += beam_weight)
                        .or_insert(*beam_weight);
                    new_beams
                        .entry(right)
                        .and_modify(|e| *e += beam_weight)
                        .or_insert(*beam_weight);
                }
                // Propagate
                _ => {
                    new_beams
                        .entry(next_coords)
                        .and_modify(|e| *e += beam_weight)
                        .or_insert(*beam_weight);
                }
            }
        }

        beams = new_beams;
    }

    timeline_count.to_string()
}
