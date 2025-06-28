use crate::day14::model::SecurityRobot;
use crate::toolbox::Coordinates;
use itertools::all;
use std::collections::HashSet;

// Part one
const ELAPSED_TIME_P1: i32 = 100;

const X_AXIS_LENGTH: i32 = 101; // Input: 101, test: 11
const Y_AXIS_LENGTH: i32 = 103; // Input: 103, test: 07

pub fn solve_part_one(robots: &[SecurityRobot]) -> i32 {
    let x_half_length: i32 = X_AXIS_LENGTH / 2;
    let y_half_length: i32 = Y_AXIS_LENGTH / 2;

    let accumulators: (i32, i32, i32, i32) = robots
        .iter()
        .map(|robot| robot.progress_for(ELAPSED_TIME_P1))
        .map(|coord| Coordinates {
            x: ((coord.x % X_AXIS_LENGTH) + X_AXIS_LENGTH) % X_AXIS_LENGTH,
            y: ((coord.y % Y_AXIS_LENGTH) + Y_AXIS_LENGTH) % Y_AXIS_LENGTH,
        })
        .fold((0, 0, 0, 0), |(acc0, acc1, acc2, acc3), value| {
            if value.x < x_half_length && value.y < y_half_length {
                (acc0 + 1, acc1, acc2, acc3)
            } else if value.x < x_half_length && value.y > y_half_length {
                (acc0, acc1 + 1, acc2, acc3)
            } else if value.x > x_half_length && value.y < y_half_length {
                (acc0, acc1, acc2 + 1, acc3)
            } else if value.x > x_half_length && value.y > y_half_length {
                (acc0, acc1, acc2, acc3 + 1)
            } else {
                (acc0, acc1, acc2, acc3)
            }
        });

    accumulators.0 * accumulators.1 * accumulators.2 * accumulators.3
}

/// [[SPOILER AHEAD]]
///
/// The "picture" of the tree is full, and in a rectangle.
/// This means that most of the robots are gathered.
///
/// This functions look for the first time the robot are highly condensed.
pub fn solve_part_two(robots: &[SecurityRobot]) -> i32 {
    let x_quarter_length: i32 = X_AXIS_LENGTH / 4;
    let y_quarter_length: i32 = Y_AXIS_LENGTH / 4;

    let mut time: i32 = 0;
    loop {
        let robot_coordinates: Vec<Coordinates> = robots
            .iter()
            .map(|robot| robot.progress_for(time))
            .map(|coord| Coordinates {
                x: ((coord.x % X_AXIS_LENGTH) + X_AXIS_LENGTH) % X_AXIS_LENGTH,
                y: ((coord.y % Y_AXIS_LENGTH) + Y_AXIS_LENGTH) % Y_AXIS_LENGTH,
            })
            .collect();

        let len: i32 = robot_coordinates.len() as i32;

        // Compute average position
        let avg_x: i32 = robot_coordinates.iter().map(|coord| coord.x).sum::<i32>() / len;
        let avg_y: i32 = robot_coordinates.iter().map(|coord| coord.y).sum::<i32>() / len;

        // Compute variance
        let variance_x: i32 = robot_coordinates
            .iter()
            .map(|coord| (coord.x - avg_x).pow(2))
            .sum::<i32>()
            / len;
        let variance_y: i32 = robot_coordinates
            .iter()
            .map(|coord| (coord.y - avg_y).pow(2))
            .sum::<i32>()
            / len;

        if variance_x <= x_quarter_length.pow(2) && variance_y <= y_quarter_length.pow(2) {
            display_robots(robot_coordinates);
            break;
        } else {
            time += 1
        }
    }

    time
}

pub fn display_robots(robot_coordinates: Vec<Coordinates>) {
    let mut debug_grid = vec![vec![0; Y_AXIS_LENGTH as usize]; X_AXIS_LENGTH as usize];
    for coord in robot_coordinates.iter() {
        debug_grid[coord.x as usize][coord.y as usize] += 1;
    }

    let mut grid_str = String::new();
    for i in 0..debug_grid.len() {
        for j in 0..debug_grid.first().unwrap().len() {
            let val = debug_grid[i][j];
            let string: String = match val {
                0 => String::from("."),
                _ => val.to_string(),
            };
            grid_str.push_str(&string);
        }
        grid_str.push('\n')
    }
    println!("{}", grid_str);
}
