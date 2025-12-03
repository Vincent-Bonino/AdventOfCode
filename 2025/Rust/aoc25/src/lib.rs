#![allow(unused_imports)]

pub mod prelude;

pub mod cli;
pub mod solution;
pub mod tools;

pub mod day01;
pub mod day02;
pub mod day03;

/// Number of puzzle to solve this year.
pub static DAY_NUMBER: usize = 12;
/// Targeted total duration.
pub static TARGET_MS: u128 = 1000;
