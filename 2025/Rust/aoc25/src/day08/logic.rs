use std::cmp::{Ordering, PartialOrd};

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use super::model::Point;

const CONNECTION_COUNT_P1: usize = 1000;

pub fn solve_part_one(distances: &[(usize, usize, f32)]) -> String {
    // Count selected points
    let selected = &distances[0..CONNECTION_COUNT_P1];

    let selected_points: HashSet<usize> = selected.iter().flat_map(|(p1, p2, _)| [p1, p2]).copied().collect();
    let selected_points_number: usize = selected_points.len();

    // Build graph

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::with_capacity(selected_points_number);
    for (p1, p2, _) in selected {
        graph.entry(*p1).or_insert(Vec::new()).push(*p2);
        graph.entry(*p2).or_insert(Vec::new()).push(*p1);
    }

    // Count components
    let mut component_lengths = compute_components_lengths(&graph, &selected_points);

    component_lengths.sort_by(|a, b| b.cmp(a));
    component_lengths.iter().take(3).product::<usize>().to_string()
}

pub fn solve_part_two(points: &[Point], distances: &[(usize, usize, f32)]) -> String {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::with_capacity(points.len());
    let mut selected_points: HashSet<usize> = HashSet::with_capacity(points.len());

    let mut result_index: Option<usize> = None;

    for (distance_index, (p1, p2, _)) in distances.iter().enumerate() {
        // Append to selected points
        selected_points.insert(*p1);
        selected_points.insert(*p2);

        // Append to graph
        graph.entry(*p1).or_insert(Vec::new()).push(*p2);
        graph.entry(*p2).or_insert(Vec::new()).push(*p1);

        let component_lengths = compute_components_lengths(&graph, &selected_points);
        if let Some(len) = component_lengths.first()
            && len == &points.len()
        {
            result_index = Some(distance_index);
            break;
        }
    }

    let result_index: usize = result_index.expect("Result index not found");

    let (l_index, r_index, _) = distances[result_index];
    (points[l_index].0 as u64 * points[r_index].0 as u64).to_string()
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

// Graph

/// Taking a graph, and the points being processed, compute the number of graph components and their length.
fn compute_components_lengths(graph: &HashMap<usize, Vec<usize>>, point_indexes: &HashSet<usize>) -> Vec<usize> {
    let mut component_lengths: Vec<usize> = Vec::new();
    let mut visited: HashSet<usize> = HashSet::with_capacity(point_indexes.len());

    for point in point_indexes {
        if visited.contains(point) {
            continue;
        }

        component_lengths.push(visit(graph, *point, &mut visited, 0));
    }

    component_lengths
}

/// Visit a graph.
///
/// Used to count the number of graph components and their size.
fn visit(graph: &HashMap<usize, Vec<usize>>, point: usize, visited: &mut HashSet<usize>, count: usize) -> usize {
    let mut result = count + 1;
    visited.insert(point);

    for adj in graph.get(&point).unwrap() {
        if !visited.contains(adj) {
            result += visit(graph, *adj, visited, count);
        }
    }

    result
}
