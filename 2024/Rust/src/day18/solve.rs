use crate::day18::dijkstra::custom_dijkstra;
use crate::day18::model::Tile18;
use crate::toolbox::{Coordinates, Grid};

const PART_ONE_LIMIT: usize = 1024;
#[allow(dead_code)]
const PART_ONE_TEST_LIMIT: usize = 12;

const USED_LIMIT: usize = PART_ONE_LIMIT;

pub fn solve_part_one(falling_bytes_coordinates: &[Coordinates], axis_length: usize) -> i128 {
    let mut grid = Grid::fill(Tile18::Empty, axis_length, axis_length);

    // Make the bytes fall
    for coord in falling_bytes_coordinates[0..USED_LIMIT].iter() {
        grid.replace(coord, Tile18::Corrupted);
    }

    let start: Coordinates = Coordinates { x: 0, y: 0 };
    let finish: Coordinates = Coordinates {
        x: axis_length as i32 - 1,
        y: axis_length as i32 - 1,
    };

    let (distance, _path) = custom_dijkstra(&grid, &start, &finish);

    distance as i128
}

pub fn solve_part_two(falling_bytes_coordinates: &[Coordinates], axis_length: usize) -> i128 {
    let mut grid = Grid::fill(Tile18::Empty, axis_length, axis_length);

    // Make the bytes fall
    for coord in falling_bytes_coordinates[0..USED_LIMIT].iter() {
        grid.replace(coord, Tile18::Corrupted);
    }

    let start: Coordinates = Coordinates { x: 0, y: 0 };
    let finish: Coordinates = Coordinates {
        x: axis_length as i32 - 1,
        y: axis_length as i32 - 1,
    };

    let (mut distance, mut path) = custom_dijkstra(&grid, &start, &finish);
    let mut next_byte_index: usize = USED_LIMIT;

    while distance != i64::MAX {
        let new_byte_coord: &Coordinates = &falling_bytes_coordinates[next_byte_index];
        grid.replace(new_byte_coord, Tile18::Corrupted);
        next_byte_index += 1;

        if path.contains(new_byte_coord) {
            (distance, path) = custom_dijkstra(&grid, &start, &finish);
        }
    }

    let final_byte = &falling_bytes_coordinates[next_byte_index - 1];
    println!("This byte fell and blocked the last path:");
    println!("{},{}\n", final_byte.x, final_byte.y);

    // Dummy value
    1234
}
