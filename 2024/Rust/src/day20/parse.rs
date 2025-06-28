use crate::day20::model::Tile20;
use crate::toolbox::{Coordinates, Grid};
use std::arch::x86_64::CpuidResult;

const CAPACITY: usize = 141;

pub fn parse_input(input: &str) -> (Grid<Tile20>, Coordinates, Coordinates) {
    let mut data: Vec<Vec<Tile20>> = Vec::with_capacity(CAPACITY);
    let mut start_position: Option<Coordinates> = None;
    let mut end_position: Option<Coordinates> = None;

    for line in input.lines() {
        let mut sub_data: Vec<Tile20> = Vec::new();
        for chr in line.chars() {
            match chr {
                '.' => sub_data.push(Tile20::Empty),
                '#' => sub_data.push(Tile20::Wall),
                'S' => {
                    start_position = Some(Coordinates {
                        x: data.len() as i32,
                        y: sub_data.len() as i32,
                    });
                    sub_data.push(Tile20::Empty);
                }
                'E' => {
                    end_position = Some(Coordinates {
                        x: data.len() as i32,
                        y: sub_data.len() as i32,
                    });
                    sub_data.push(Tile20::Empty);
                }
                _ => unreachable!(),
            }
        }
        data.push(sub_data)
    }

    (
        Grid::new(data),
        start_position.expect("No start"),
        end_position.expect("No end"),
    )
}
