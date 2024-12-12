use crate::toolbox::{Coordinates, Direction};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

pub struct Region {
    pub plant_type: char,
    coordinates: Vec<Coordinates>,

    perimeter: i32,
}

/// Represent an Edge, a border between two coordinates.
///
/// The facing direction points to the Coordinate from which it originates.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Edge {
    pub x: f32,
    pub y: f32,
    pub facing: Direction,
}

impl Edge {
    pub fn from_coords(inner_coord: &Coordinates, outer_coord: &Coordinates) -> Self {
        Self {
            x: (inner_coord.x as f32 + outer_coord.x as f32) / 2_f32,
            y: (inner_coord.y as f32 + outer_coord.y as f32) / 2_f32,
            facing: Direction::from_delta(inner_coord.get_delta(outer_coord)),
        }
    }

    pub fn step(&self, direction: &Direction) -> Self {
        let delta: (i32, i32) = direction.get_delta();
        Self {
            x: self.x + delta.0 as f32,
            y: self.y + delta.1 as f32,
            facing: self.facing,
        }
    }
}

impl Region {
    pub fn new(plant_type: char) -> Self {
        Region {
            plant_type,
            coordinates: Vec::new(),
            perimeter: 0,
        }
    }

    /// Insert a new coordinate to the region.
    /// Dynamically update the perimeter of the region.
    pub fn add_coordinates(&mut self, coordinates: Coordinates) {
        self.perimeter += match self.known_neighbours(&coordinates).len() {
            0 => 4,  // First coordinate of the region
            1 => 2,  // Replace 1 side by 3
            2 => 0,  // Replace 2 sides by 2
            3 => -2, // Replace 3 sides by 1
            4 => -4, // Replace 4 sides by 0
            _ => unreachable!(),
        };

        self.coordinates.push(coordinates);
    }

    pub fn area(&self) -> i32 {
        self.coordinates.len() as i32
    }

    pub fn perimeter(&self) -> i32 {
        self.perimeter
    }

    pub fn side_number(&self) -> i32 {
        // Compute region edges
        let edges: Vec<Edge> = self
            .coordinates
            .iter()
            .flat_map(|inner_coord| {
                self.unknown_neighbours(inner_coord)
                    .into_iter()
                    .map(|outer_coord| Edge::from_coords(inner_coord, &outer_coord))
            })
            .collect();

        // Transform edges into sides
        let mut used_edges: Vec<Edge> = Vec::new();
        let mut side_counter: i32 = 0;

        for edge in edges.iter() {
            if used_edges.contains(edge) {
                continue;
            }
            used_edges.push(*edge);

            // Build a side, and use all its edges
            side_counter += 1;

            for dir in Direction::neighbours4() {
                let mut curr_edge: Edge = edge.step(&dir);

                while edges.contains(&curr_edge) && !used_edges.contains(&curr_edge) {
                    used_edges.push(curr_edge);
                    curr_edge = curr_edge.step(&dir);
                }
            }
        }

        side_counter
    }

    // Utils

    /// Return a [`Vec`] of neighbour(s) contained in the region.
    fn known_neighbours(&self, coordinates: &Coordinates) -> Vec<Coordinates> {
        coordinates
            .neighbours4()
            .into_iter()
            .filter(|neighbour| self.coordinates.contains(neighbour))
            .collect()
    }

    /// Return a [`Vec`] of neighbour(s) not contained in the region.
    fn unknown_neighbours(&self, coordinates: &Coordinates) -> Vec<Coordinates> {
        coordinates
            .neighbours4()
            .into_iter()
            .filter(|neighbour| !self.coordinates.contains(neighbour))
            .collect()
    }
}

impl Debug for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Region<{}; a={},p={},s={}; {:?}>",
            self.plant_type,
            self.area(),
            self.perimeter(),
            self.side_number(),
            self.coordinates,
        )
    }
}
