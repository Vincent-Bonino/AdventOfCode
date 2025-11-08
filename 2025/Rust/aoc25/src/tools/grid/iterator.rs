use super::Grid;
use crate::coord;
use crate::tools::{coordinates::Coordinates, types::CoordIntType};
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

// -- Iterator --

pub struct GridIter<'g, T> {
    grid: &'g Grid<T>,
    data_iter: Iter<'g, T>,
}

impl<'g, T> GridIter<'g, T> {
    pub(super) fn new(grid: &'g Grid<T>) -> Self {
        Self {
            grid,
            data_iter: grid.data.iter(),
        }
    }
}

impl<'g, T> Iterator for GridIter<'g, T> {
    type Item = &'g T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data_iter.next()
    }
}

impl<'g, T> IntoIterator for &'g Grid<T> {
    type Item = &'g T;
    type IntoIter = GridIter<'g, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// -- IteratorMut --

pub struct GridIterMut<'g, T> {
    data_iter_mut: IterMut<'g, T>,

    grid_rows: usize,
    grid_cols: usize,
}

impl<'g, T> GridIterMut<'g, T> {
    pub(super) fn new(grid: &'g mut Grid<T>) -> Self {
        Self {
            data_iter_mut: grid.data.iter_mut(),
            grid_rows: grid.rows,
            grid_cols: grid.cols,
        }
    }
}

impl<'g, T> Iterator for GridIterMut<'g, T> {
    type Item = &'g mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data_iter_mut.next()
    }
}

impl<'g, T> IntoIterator for &'g mut Grid<T> {
    type Item = &'g mut T;
    type IntoIter = GridIterMut<'g, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// -- IntoIterator --

pub struct GridIntoIter<T> {
    data_into_iter: IntoIter<T>,

    grid_rows: usize,
    grid_cols: usize,
}

impl<T> Iterator for GridIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data_into_iter.next()
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = GridIntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            grid_rows: self.rows,
            grid_cols: self.cols,
            data_into_iter: self.data.into_iter(),
        }
    }
}

// -- CoordEnumerate --

pub struct CoordEnumerate<I> {
    iter: I,
    count: usize,

    rows: usize,
    cols: usize,
}

impl<I> CoordEnumerate<I> {
    pub fn new(iter: I, rows: usize, cols: usize) -> Self {
        Self {
            iter,
            rows,
            cols,
            count: 0,
        }
    }

    #[inline]
    fn revert_index(&self, index: usize) -> (usize, usize) {
        (index / self.cols, index % self.rows)
    }
}

impl<I> Iterator for CoordEnumerate<I>
where
    I: Iterator,
{
    type Item = (Coordinates, <I as Iterator>::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let value: <I as Iterator>::Item = self.iter.next()?;
        let (r, c): (usize, usize) = self.revert_index(self.count);
        self.count += 1;

        Some((coord!(r, c), value))
    }
}

/// Similar to [`Enumerate`](std::iter::Enumerate) for a [`Grid`] but indices are [`Coordinates`].
pub trait CoordEnumerator {
    type Iterator: Iterator;

    /// "Enumerate" over the values of a grid, but instead of returning their index alongside the values,
    /// return their [`Coordinates`].
    fn coord_enumerate(self) -> CoordEnumerate<Self::Iterator>;
}

impl<'g, T> CoordEnumerator for GridIter<'g, T> {
    type Iterator = Iter<'g, T>;

    fn coord_enumerate(self) -> CoordEnumerate<Self::Iterator> {
        CoordEnumerate::new(self.data_iter, self.grid.rows, self.grid.cols)
    }
}

impl<'g, T> CoordEnumerator for GridIterMut<'g, T> {
    type Iterator = IterMut<'g, T>;

    fn coord_enumerate(self) -> CoordEnumerate<Self::Iterator> {
        CoordEnumerate::new(self.data_iter_mut, self.grid_rows, self.grid_cols)
    }
}

impl<T> CoordEnumerator for GridIntoIter<T> {
    type Iterator = IntoIter<T>;

    fn coord_enumerate(self) -> CoordEnumerate<Self::Iterator> {
        CoordEnumerate::new(self.data_into_iter, self.grid_rows, self.grid_cols)
    }
}
