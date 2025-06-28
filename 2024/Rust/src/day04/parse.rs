use crate::day04::model::Letter;
use crate::toolbox::Grid;

pub fn parse_input(input: &str) -> Grid<Letter> {
    let mut data: Vec<Vec<Letter>> = Vec::with_capacity(140);

    for line in input.lines() {
        let mut sub_data: Vec<Letter> = Vec::new();
        for chr in line.chars() {
            match chr {
                'X' => sub_data.push(Letter::X),
                'M' => sub_data.push(Letter::M),
                'A' => sub_data.push(Letter::A),
                'S' => sub_data.push(Letter::S),
                _ => sub_data.push(Letter::Irrelevant),
            }
        }
        data.push(sub_data)
    }

    Grid::new(data)
}
