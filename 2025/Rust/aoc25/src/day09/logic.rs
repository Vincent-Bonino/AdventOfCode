use std::collections::VecDeque;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use super::model::Line;
use crate::coord;
use crate::prelude::*;

type CoordShrinkMapping = HashMap<CoordIntType, CoordIntType>;

pub fn solve_part_one(coordinates: &[Coordinates]) -> String {
    coordinates
        .iter()
        .tuple_combinations()
        .map(|(lhs, rhs)| {
            let h = (lhs.x - rhs.x).abs() + 1;
            let w = (lhs.y - rhs.y).abs() + 1;
            h as i64 * w as i64
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn solve_part_two(coordinates: &[Coordinates]) -> String {
    let (x_mapping, y_mapping, shrunk_coordinates) = shrink_space(coordinates);

    //
    // Everything below happens in shrunk space
    //

    // Compute the red/green lines
    let red_green_lines: Vec<Line> = shrunk_coordinates
        .iter()
        .circular_tuple_windows()
        .map(|(lhs, rhs)| Line::new(*lhs, *rhs))
        .collect();

    let outside_coords: HashSet<Coordinates> = flood_fill(&shrunk_coordinates, red_green_lines);
    let all_rectangles: Vec<(Coordinates, Coordinates)> =
        shrunk_coordinates.iter().copied().tuple_combinations().collect();

    let valid_rectangles: Vec<(Coordinates, Coordinates)> = all_rectangles
        .iter()
        .filter(|rect| filter_rectangle(rect, &outside_coords))
        .copied()
        .collect();

    let expanded_valid_rectangles: Vec<(Coordinates, Coordinates)> = valid_rectangles
        .iter()
        .map(|(lhs, rhs)| {
            (
                expand_coord(*lhs, &x_mapping, &y_mapping),
                expand_coord(*rhs, &x_mapping, &y_mapping),
            )
        })
        .collect();
    //
    // Everything below happens in expanded (normal) space
    //

    expanded_valid_rectangles
        .iter()
        .map(|(lhs, rhs)| {
            let h = (lhs.x - rhs.x).abs() + 1;
            let w = (lhs.y - rhs.y).abs() + 1;
            h as i64 * w as i64
        })
        .max()
        .unwrap()
        .to_string()
}

/// Shrink space, only keeping the shapes but not the size.
fn shrink_space(coordinates: &[Coordinates]) -> (CoordShrinkMapping, CoordShrinkMapping, Vec<Coordinates>) {
    let mut base_xs: Vec<CoordIntType> = coordinates.iter().map(|c| c.x).collect();
    let mut base_ys: Vec<CoordIntType> = coordinates.iter().map(|c| c.y).collect();

    base_xs.sort();
    base_ys.sort();

    let x_mapping: CoordShrinkMapping = base_xs
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i as CoordIntType))
        .collect();
    let y_mapping: CoordShrinkMapping = base_ys
        .iter()
        .enumerate()
        .map(|(i, &y)| (y, i as CoordIntType))
        .collect();

    let mapped_coordinates: Vec<Coordinates> = coordinates
        .iter()
        .map(|coord| {
            let nx = x_mapping[&coord.x];
            let ny = y_mapping[&coord.y];
            coord!(nx, ny)
        })
        .collect();

    let x_mapping: CoordShrinkMapping = x_mapping.into_iter().map(|(x, i)| (i, x)).collect();
    let y_mapping: CoordShrinkMapping = y_mapping.into_iter().map(|(y, i)| (i, y)).collect();

    (x_mapping, y_mapping, mapped_coordinates)
}

#[inline]
fn expand_coord(
    coordinates: Coordinates,
    x_mapping: &CoordShrinkMapping,
    y_mapping: &CoordShrinkMapping,
) -> Coordinates {
    coord!(x_mapping[&coordinates.x], y_mapping[&coordinates.y])
}

/// Using flood fill, compute the coordinates *outside* the provided shape(s).
fn flood_fill(coordinates: &[Coordinates], borders: Vec<Line>) -> HashSet<Coordinates> {
    let borders: HashSet<Coordinates> = borders.into_iter().flat_map(|line| line.to_coordinates()).collect();

    let min_x = coordinates.iter().map(|c| c.x).min().unwrap() - 1;
    let max_x = coordinates.iter().map(|c| c.x).max().unwrap() + 1;
    let min_y = coordinates.iter().map(|c| c.y).min().unwrap() - 1;
    let max_y = coordinates.iter().map(|c| c.y).max().unwrap() + 1;

    let x_range = max_x - min_x;
    let y_range = max_y - min_y;

    let mut outside: HashSet<Coordinates> = HashSet::with_capacity((x_range * y_range) as usize);

    let mut queue: VecDeque<Coordinates> = VecDeque::new();
    let mut seen: HashSet<Coordinates> = HashSet::with_capacity((x_range * y_range) as usize);

    queue.push_back(coord!(min_x, min_y));

    while let Some(coord) = queue.pop_front() {
        // Stop conditions
        if seen.contains(&coord) {
            continue; // Already processed
        }
        seen.insert(coord);

        if coord.x < min_x || coord.x > max_x || coord.y < min_y || coord.y > max_y {
            continue; // Out of bounds
        } else if borders.contains(&coord) {
            continue; // Entering a shape
        }

        outside.insert(coord);
        queue.extend(coord.neighbours4())
    }

    outside
}

fn build_rectangle_sides((coord1, coord3): &(Coordinates, Coordinates)) -> [Line; 4] {
    let coord2 = coord!(coord1.x, coord3.y);
    let coord4 = coord!(coord3.x, coord1.y);

    [
        Line::new(*coord1, coord2),
        Line::new(coord2, *coord3),
        Line::new(*coord3, coord4),
        Line::new(coord4, *coord1),
    ]
}

fn filter_rectangle(rectangle: &(Coordinates, Coordinates), outside: &HashSet<Coordinates>) -> bool {
    for line in build_rectangle_sides(rectangle) {
        for coord in line.to_coordinates() {
            if outside.contains(&coord) {
                return false;
            }
        }
    }

    // All is good
    true
}
