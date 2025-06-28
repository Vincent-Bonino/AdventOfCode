use crate::day08::model::Tile08;
use crate::toolbox::Grid;

pub fn parse_input(input: &str) -> Grid<Tile08> {
    let mut data: Vec<Vec<Tile08>> = Vec::with_capacity(140);

    for line in input.lines() {
        let mut sub_data: Vec<Tile08> = Vec::new();
        for chr in line.chars() {
            match chr {
                '.' => sub_data.push(Tile08::Empty),
                _ => sub_data.push(Tile08::Antenna(chr)),
            }
        }
        data.push(sub_data)
    }

    Grid::new(data)
}
