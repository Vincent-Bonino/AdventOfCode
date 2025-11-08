//! Module containing several utilities to represent and work with a 2D [`grid`](Grid) of values.

mod implementations;
mod iterator;
mod structure;

// Public re-exports
pub use iterator::CoordEnumerator;
pub use structure::Grid;
