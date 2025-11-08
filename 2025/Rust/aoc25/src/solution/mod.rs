//! Traits used to generalize solutions.

mod aoc_day;
mod aoc_result;
mod aoc_solution;
mod benchmark;

// Public re-exports
pub use aoc_day::Aoc25Day;
pub use aoc_result::Aoc25Result;
pub use aoc_solution::Aoc25Solution;

#[cfg(feature = "benchmark")]
pub use aoc_result::format_duration;
