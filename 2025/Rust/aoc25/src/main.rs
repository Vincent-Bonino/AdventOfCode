use std::process::exit;

use clap::Parser;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use paste::paste;

use aoc25::cli::{AllDaysArgs, Aoc25Args, Aoc25Command, SingleDayArgs};
use aoc25::solution::Aoc25Result;
use aoc25::solution::Aoc25Solution;
#[cfg(feature = "benchmark")]
use aoc25::solution::format_duration;

macro_rules! build_solution_index {
    // Base case, should not be called directly
    (@box $number:expr) => {
        paste!( Box::new(aoc25::[<day$number>]::[<Day$number>]::default()) )
    };

    // All provided days
    ( $( $number:literal ),* $(,)? ) => {
        vec![ $( build_solution_index!(@box $number), )* ]
    };
}

fn main() {
    println!("Advent of Code 2025!\n");

    // Create index of solutions
    let solutions: Vec<Box<dyn Aoc25Solution>> = build_solution_index!(01, 02, 03);

    let args: Aoc25Args = Aoc25Args::parse();
    let result = match args.command {
        Aoc25Command::SingleDay(args) => run_single_day(args, solutions),
        Aoc25Command::AllDays(args) => run_all_days(args, solutions),
    };

    match result {
        Ok(_) => exit(0),
        Err(_) => exit(1),
    }
}

fn run_single_day(args: SingleDayArgs, solutions: Vec<Box<dyn Aoc25Solution>>) -> Result<(), ()> {
    // Find solution
    let solution: Option<Box<dyn Aoc25Solution>> = solutions.into_iter().find(|sol| sol.get_day_number() == args.day);

    // Run solution
    match solution {
        Some(mut sol) => {
            let result: Aoc25Result = sol.run(args.use_test, args.test_extra);
            println!("{}", result);
            Ok(())
        }
        None => {
            println!("No solution found for day {}", args.day);
            Err(())
        }
    }
}

#[cfg(feature = "benchmark")]
fn run_all_days(_args: AllDaysArgs, solutions: Vec<Box<dyn Aoc25Solution>>) -> Result<(), ()> {
    let solution_count: usize = solutions.len();
    let mut total_time: u128 = 0;

    let mut result_table: Table = Table::new();
    let headers: Vec<&str> = vec![
        "Day",
        "Part 1",
        "Part 2",
        "Parsing",
        "Solving part 1",
        "Solving part 2",
        "Total",
    ];
    result_table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(headers);

    //
    // Run
    //

    for mut solution in solutions {
        let res: Aoc25Result = solution.run(false, None);

        total_time += res.get_total_duration();
        result_table.add_row(res);
    }

    println!("Notes:");
    println!(" - All times are in microseconds.");
    println!(" - File loading times are excluded.");

    println!("{}", result_table);
    println!("Total time: {}", format_duration(total_time));
    println!(
        "Average time per day: {}",
        format_duration(total_time / (solution_count as u128))
    );

    Ok(())
}

#[cfg(not(feature = "benchmark"))]
fn run_all_days(_args: AllDaysArgs, solutions: Vec<Box<dyn Aoc25Solution>>) -> Result<(), ()> {
    let mut result_table: Table = Table::new();
    let headers: Vec<&str> = vec!["Day", "Part 1", "Part 2"];
    result_table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(headers);

    //
    // Run
    //

    for mut solution in solutions {
        let res: Aoc25Result = solution.run(true, None);
        result_table.add_row(res);
    }

    println!("{}", result_table);
    Ok(())
}
