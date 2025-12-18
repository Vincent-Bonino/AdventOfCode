use std::cmp::{Ordering, PartialOrd};

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use super::model::Point;

const CONNECTION_COUNT_P1: usize = 1000;
const CIRCUIT_COUNT_P1: usize = 3;

pub fn solve_both_parts(points: &[Point], distances: &[(usize, usize, f32)]) -> (String, String) {
    let mut part1: String = String::new();
    let mut part2: String = String::new();

    let nb_points: usize = points.len();

    let mut data: HashMap<usize, usize> = (0..nb_points).map(|i| (i, i)).collect();
    let mut nb_connections: usize = 0;

    // Connect junction boxes
    for (index, (p1, p2, _)) in distances.iter().enumerate() {
        let find1: usize = find(&mut data, *p1);
        let find2: usize = find(&mut data, *p2);

        if find1 != find2 {
            // Creating a new connection
            nb_connections += 1;

            if nb_connections == nb_points - 1 {
                // All points are connected (P2)
                part2 = (points[*p1].0 as u64 * points[*p2].0 as u64).to_string();
            }

            union(&mut data, *p1, *p2);
        }

        if (index + 1) == CONNECTION_COUNT_P1 {
            part1 = compute_part_one(&mut data, nb_points);
        }
    }

    (part1, part2)
}

/// Measure circuits
fn compute_part_one(data: &mut HashMap<usize, usize>, nb_points: usize) -> String {
    let mut sizes: HashMap<usize, usize> = (0..nb_points).map(|i| (i, 0)).collect();

    for i in 0..nb_points {
        let circuit_id: usize = find(data, i);
        *sizes.get_mut(&circuit_id).unwrap() += 1;
    }

    sizes
        .values()
        .sorted_by_key(|v| -(**v as isize)) // Reverse sort
        .map(|v| *v as u64)
        .take(CIRCUIT_COUNT_P1)
        .product::<u64>()
        .to_string()
}

// Prepare

pub fn compute_distances(points: &[Point]) -> Vec<(usize, usize, f32)> {
    let mut distances: Vec<(usize, usize, f32)> = points
        .iter()
        .enumerate()
        .cartesian_product(points.iter().enumerate())
        .filter_map(|((left_index, left_point), (right_index, right_point))| {
            if left_index < right_index {
                Some((left_index, right_index, left_point.distance_to(right_point)))
            } else {
                None
            }
        })
        .collect();
    distances.sort_by(|(_, _, dl), (_, _, dr)| dl.partial_cmp(dr).unwrap());
    distances
}

// Union-find

fn find(data: &mut HashMap<usize, usize>, value: usize) -> usize {
    if value == data[&value] {
        value
    } else {
        let new_value: usize = find(data, data[&value]);
        *data.get_mut(&value).unwrap() = new_value;
        new_value
    }
}

fn union(data: &mut HashMap<usize, usize>, value1: usize, value2: usize) {
    let find1 = find(data, value1);
    let find2 = find(data, value2);
    *data.get_mut(&find1).unwrap() = find2;
}
