use crate::day16::model::Tile16;
use crate::toolbox::{Coordinates, Grid};

const GRID_SIZE: usize = 141; // From input

pub fn parse_input(input: &str) -> (Grid<Tile16>, Coordinates, Coordinates) {
    let mut data: Vec<Vec<Tile16>> = Vec::with_capacity(GRID_SIZE);
    let mut start_pos: Option<Coordinates> = None;
    let mut exit_pos: Option<Coordinates> = None;

    for (line_index, line) in input.lines().enumerate() {
        let mut data_line: Vec<Tile16> = Vec::with_capacity(GRID_SIZE);

        for chr in line.chars() {
            match chr {
                'S' => {
                    start_pos = Some(Coordinates {
                        x: line_index as i32,
                        y: data_line.len() as i32,
                    });
                    data_line.push(Tile16::Empty);
                }
                'E' => {
                    exit_pos = Some(Coordinates {
                        x: line_index as i32,
                        y: data_line.len() as i32,
                    });
                    data_line.push(Tile16::Empty);
                }
                '.' => data_line.push(Tile16::Empty),
                '#' => data_line.push(Tile16::Wall),
                _ => unreachable!(),
            }
        }
        data.push(data_line)
    }

    (
        Grid::new(data),
        start_pos.expect("No starting position"),
        exit_pos.expect("No exit position"),
    )
}
