#![allow(unused_imports)]

pub mod prelude;

pub mod cli;
pub mod solution;
pub mod tools;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
// pub mod day09;
// pub mod day10;
pub mod day11;

/// Number of puzzle to solve this year.
pub static DAY_NUMBER: usize = 12;
/// Targeted total duration.
pub static TARGET_MS: u128 = 1000;
