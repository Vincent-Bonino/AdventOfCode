use crate::day04::model::Letter;
use crate::toolbox::{Coordinates, Grid};

pub fn solve_part_one(grid: &Grid<Letter>) -> i128 {
    let mut counter: i32 = 0;
    let word: [Letter; 4] = [Letter::X, Letter::M, Letter::A, Letter::S];

    for i in 0..grid.depth() {
        for j in 0..grid.width() {
            counter += grid.count_xmas(
                &Coordinates {
                    x: i as i32,
                    y: j as i32,
                },
                &word,
            )
        }
    }

    counter as i128
}

pub fn solve_part_two(grid: &Grid<Letter>) -> i128 {
    let mut counter: i32 = 0;

    for i in 0..grid.depth() {
        for j in 0..grid.width() {
            let coord: Coordinates = Coordinates {
                x: i as i32,
                y: j as i32,
            };
            if grid.is_x_max(&coord) {
                counter += 1;
            }
        }
    }

    counter as i128
}
