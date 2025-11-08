#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use colored::{ColoredString, Colorize};

use super::iterator::{GridIter, GridIterMut};
use crate::coord;
use crate::tools::coordinates::Coordinates;
use crate::tools::types::CoordIntType;

/// Grid of data in two dimensions.
///
/// Made to work with [`Coordinates`]
///
/// Orientation:
/// ```text
/// +--------> c  (columns)
/// |
/// |
/// V
/// r (rows)
/// ```
#[derive(Clone, Debug, Default)]
pub struct Grid<T> {
    /// Store the grid as a single vector.
    pub data: Vec<T>,

    /// Number of rows.
    pub rows: usize,
    /// Number of columns.
    pub cols: usize,
}

impl<T> Grid<T> {
    /// Build a Grid from nested vectors.
    pub fn from_nested(data: Vec<Vec<T>>) -> Self {
        Self {
            rows: data.len(),
            cols: data[0].len(),
            data: Vec::from_iter(data.into_iter().flatten()),
        }
    }

    /// Build an empty grid with `rows` * `cols` capacity.
    pub fn with_capacity(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: Vec::with_capacity(rows * cols),
        }
    }

    // Indexes

    #[inline]
    pub(super) fn convert_index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    #[inline]
    pub(super) fn revert_index(&self, index: usize) -> (usize, usize) {
        (index % self.rows, index / self.cols)
    }

    // Bounds check

    #[inline]
    /// Determines if a coordinates represent a value of the grid.
    pub fn is_in_bounds(&self, coordinates: &Coordinates) -> bool {
        let (r, c): (usize, usize) = coordinates.as_usize_tuple_unchecked();
        self._is_in_bounds(r, c)
    }

    #[inline]
    /// Determines if a coordinates represent a value of the grid.
    fn _is_in_bounds(&self, row: usize, col: usize) -> bool {
        // Note: usize can't be below 0
        row < self.rows && col < self.cols
    }

    // Getters

    /// Get a reference to the value stored at the provided coordinates.
    ///
    /// Return None if the provided coordinate is out of bounds.
    #[inline]
    pub fn get_ref(&self, coordinates: &Coordinates) -> Option<&T> {
        match self.is_in_bounds(coordinates) {
            false => None,
            true => {
                let (r, c): (usize, usize) = coordinates.as_usize_tuple_unchecked();
                Some(&self.data[self.convert_index(r, c)])
            }
        }
    }

    /// Get a mutable reference to the value stored at the provided coordinates.
    ///
    /// Return None if the provided coordinate is out of bounds.
    #[inline]
    pub fn get_ref_mut(&mut self, coordinates: &Coordinates) -> Option<&mut T> {
        match self.is_in_bounds(coordinates) {
            false => None,
            true => {
                let (r, c): (usize, usize) = coordinates.as_usize_tuple_unchecked();
                let index: usize = self.convert_index(r, c);
                Some(&mut self.data[index])
            }
        }
    }

    /// Get a reference to the value stored at the provided coordinates.
    ///
    /// Panics if the provided coordinate is out of bounds.
    #[inline]
    pub fn get_ref_unchecked(&self, coordinates: &Coordinates) -> &T {
        let (r, c): (usize, usize) = coordinates.as_usize_tuple_unchecked();
        &self.data[self.convert_index(r, c)]
    }

    /// Get a mutable reference to the value stored at the provided coordinates.
    ///
    /// Panics if the provided coordinate is out of bounds.
    #[inline]
    pub fn get_ref_mut_unchecked(&mut self, coordinates: &Coordinates) -> &mut T {
        let (r, c): (usize, usize) = coordinates.as_usize_tuple_unchecked();
        let index: usize = self.convert_index(r, c);
        &mut self.data[index]
    }

    /// Build a vec of coordinates representing all the coordinates covered by the grid.
    ///
    /// Do not use this to iterate over the data, prefer using [`coord_enumerate`](super::CoordEnumerator::coord_enumerate)
    /// on the different grid iterators.
    pub fn get_coordinates(&self) -> Vec<Coordinates> {
        let mut coordinates: Vec<Coordinates> = Vec::with_capacity(self.data.len());

        for i in 0..self.rows {
            for j in 0..self.cols {
                coordinates.push(coord!(i, j));
            }
        }

        coordinates
    }

    // Setters

    /// Replace a value in the data.
    ///
    /// Returns `true` if the change occurred, `false` otherwise.
    pub fn replace(&mut self, coordinates: &Coordinates, value: T) -> bool {
        let (r, c): (usize, usize) = coordinates.as_usize_tuple_unchecked();

        if !self._is_in_bounds(r, c) {
            return false;
        }

        let index: usize = self.convert_index(r, c);
        self.data[index] = value;
        true
    }

    // Iteration

    pub fn iter(&self) -> GridIter<'_, T> {
        GridIter::new(self)
    }

    pub fn iter_mut(&mut self) -> GridIterMut<'_, T> {
        GridIterMut::new(self)
    }
}
