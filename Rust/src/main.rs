use std::process::exit;

use clap::Parser;

use aoc24::aoc::Aoc24Solution;
use aoc24::cli::Args;

use aoc24::day00::Day00;
use aoc24::day01::Day01;
use aoc24::day02::Day02;
use aoc24::day03::Day03;
use aoc24::day04::Day04;
use aoc24::day05::Day05;
use aoc24::day06::Day06;
use aoc24::day07::Day07;
use aoc24::day08::Day08;
use aoc24::day09::Day09;
use aoc24::day10::Day10;
use aoc24::day11::Day11;

fn main() {
    println!("Advent of Code 2024!\n");
    let args: Args = Args::parse();
    let target_day: usize = args.day;

    if target_day > 25 {
        println!("Are you joking ?");
        exit(1)
    }

    // Create index of solutions
    let mut solutions: Vec<Box<dyn Aoc24Solution>> = vec![
        Box::new(Day00::default()),
        Box::new(Day01::default()),
        Box::new(Day02::default()),
        Box::new(Day03::default()),
        Box::new(Day04::default()),
        Box::new(Day05::default()),
        Box::new(Day06::default()),
        Box::new(Day07::default()),
        Box::new(Day08::default()),
        Box::new(Day09::default()),
        Box::new(Day10::default()),
        Box::new(Day11::default()),
    ];

    // Find the right solution
    let mut found: bool = false;

    for sol in solutions.iter_mut() {
        if sol.get_day_number() != args.day {
            continue;
        }

        let part_one: i128 = sol.solve_part_one(args.use_test);
        let part_two: i128 = if part_one > 0 {
            sol.solve_part_two(args.use_test)
        } else {
            -2
        };

        println!("[Day {target_day:0>2}] Part 1: {part_one}");
        println!("[Day {target_day:0>2}] Part 2: {part_two}");

        found = true;
        break;
    }

    if !found {
        println!("No solution for day {target_day:0>2}");
        exit(1)
    }

    exit(0)
}
