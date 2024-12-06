use crate::day06::model::Tile06;
use crate::toolbox::{Coordinates, Grid};

pub fn parse_input(input: &str) -> (Grid<Tile06>, Coordinates) {
    let mut data: Vec<Vec<Tile06>> = Vec::with_capacity(140);
    let mut starting_position: Option<Coordinates> = None;

    for (line_index, line) in input.lines().enumerate() {
        let mut sub_data: Vec<Tile06> = Vec::new();
        for (chr_index, chr) in line.chars().enumerate() {
            match chr {
                '#' => sub_data.push(Tile06::Blocked),
                '.' => sub_data.push(Tile06::Free),
                '^' => {
                    starting_position = Some(Coordinates {
                        x: line_index as i32,
                        y: chr_index as i32,
                    });
                    sub_data.push(Tile06::Free)
                }
                _ => unreachable!("Unexpected input char {}", chr),
            }
        }
        data.push(sub_data)
    }

    (
        Grid::new(data),
        starting_position.expect("No starting position found"),
    )
}
