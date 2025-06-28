use crate::day15::model::Tile15;
use crate::toolbox::{Coordinates, Direction, Grid};

pub const GRID_SIZE: usize = 50; // From input

pub fn parse_input(input: &str) -> (Grid<Tile15>, Coordinates, Vec<Direction>) {
    let input_parts: Vec<&str> = input.split("\r\n\r\n").collect(); // Windows windows windows ...
    let (grid, robot_pos) = parse_grid(input_parts[0]);
    let directions = parse_directions(input_parts[1]);

    (grid, robot_pos, directions)
}

fn parse_grid(grid_lines: &str) -> (Grid<Tile15>, Coordinates) {
    let mut data: Vec<Vec<Tile15>> = Vec::with_capacity(GRID_SIZE);
    let mut robot_pos: Option<Coordinates> = None;

    for line in grid_lines.lines() {
        let mut data_line: Vec<Tile15> = Vec::with_capacity(GRID_SIZE);

        for chr in line.chars() {
            match chr {
                '.' => data_line.push(Tile15::Empty),
                'O' => data_line.push(Tile15::Box),
                '#' => data_line.push(Tile15::Wall),
                '@' => {
                    robot_pos = Some(Coordinates {
                        x: data.len() as i32,
                        y: data_line.len() as i32,
                    });
                    data_line.push(Tile15::Empty) // Robot is managed outside
                }
                _ => unreachable!(),
            }
        }
        data.push(data_line)
    }

    (Grid::new(data), robot_pos.expect("Found no robot in input"))
}

fn parse_directions(dir_lines: &str) -> Vec<Direction> {
    dir_lines
        .lines()
        .flat_map(|line| {
            line.chars().map(|chr| match chr {
                '^' => Direction::N,
                '>' => Direction::E,
                'v' => Direction::S,
                '<' => Direction::W,
                _ => unreachable!(),
            })
        })
        .collect()
}
