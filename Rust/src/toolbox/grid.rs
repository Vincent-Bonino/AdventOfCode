use crate::toolbox::coordinates::Coordinates;
use std::fmt::{format, Debug};

/// Grid of data.
///
/// Orientation:
/// +--------> y  (width)
/// |
/// |
/// V
/// x (depth)
#[derive(Default, Debug)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T: Debug> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Grid { data }
    }

    /// Get the depth of the grid (length on x-axis / number of sub-arrays)
    pub fn depth(&self) -> usize {
        self.data.len()
    }

    /// Get the width of the grid (length on y-axis / length of sub-arrays)
    pub fn width(&self) -> usize {
        self.data.first().expect("Empty grid").len()
    }

    fn is_in_bounds(&self, coordinates: &Coordinates) -> bool {
        coordinates.x >= 0
            && coordinates.y >= 0
            && (0..self.depth()).contains(&(coordinates.x as usize))
            && (0..self.width()).contains(&(coordinates.y as usize))
    }

    // Public usage

    pub fn get_ref(&self, coordinates: &Coordinates) -> Option<&T> {
        if !self.is_in_bounds(coordinates) {
            return None;
        }

        let sub_array: Option<&Vec<T>> = self.data.get(coordinates.x as usize).or(None);
        match sub_array {
            None => None,
            Some(sub_arr) => sub_arr.get(coordinates.y as usize),
        }
    }

    // Display

    pub fn show_surroundings(&self, coordinates: &Coordinates, depth: usize) {
        let min_x: i32 = coordinates.x - depth as i32;
        let max_x: i32 = coordinates.x + depth as i32;
        let min_y: i32 = coordinates.y - depth as i32;
        let max_y: i32 = coordinates.y + depth as i32;

        let mut result: String = String::new();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                match self.get_ref(&Coordinates { x, y }) {
                    None => result.push(' '),
                    Some(val) => result.push_str(&format!("{val:?}")),
                }
            }
            result.push('\n');
        }

        println!("{result}");
    }
}
