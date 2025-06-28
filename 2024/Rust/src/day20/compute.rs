use crate::day20::model::Tile20;
use crate::toolbox::{Coordinates, Grid};
use std::collections::HashMap;

pub fn compute_reachable(
    path: &[Coordinates],
    range: i64,
) -> HashMap<Coordinates, Vec<(Coordinates, i64)>> {
    let mut reachable: HashMap<Coordinates, Vec<(Coordinates, i64)>> =
        HashMap::with_capacity(path.len());

    for (s_index, start) in path.iter().enumerate() {
        for end in path[s_index + 1..].iter() {
            let dist: i64 = start.manhattan_distance_to(end) as i64;
            if dist <= range {
                reachable.entry(*start).or_default().push((*end, dist));
            }
        }
    }

    reachable
}

pub fn count_shortcuts_above(
    reachable: &HashMap<Coordinates, Vec<(Coordinates, i64)>>,
    distances: &HashMap<Coordinates, i64>,
    limit: i64,
) -> u64 {
    let mut counter: u64 = 0;

    for (sc_start, sc_end_list) in reachable.iter() {
        let dist_to_start: i64 = distances[sc_start];

        for (sc_end, sc_length) in sc_end_list.iter() {
            let end_dist_to_start: i64 = distances[sc_end];

            // Length using the regular path
            let normal_length: i64 = dist_to_start - end_dist_to_start;
            if sc_length + limit <= normal_length {
                counter += 1
            }
        }
    }

    counter
}
