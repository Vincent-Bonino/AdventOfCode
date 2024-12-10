use crate::toolbox::Grid;

const CAPACITY: usize = 45; // From input file

pub fn parse_input(input: &str) -> Grid<u32> {
    let mut data: Vec<Vec<u32>> = Vec::with_capacity(CAPACITY);

    for line in input.lines() {
        let mut sub_data: Vec<u32> = Vec::with_capacity(CAPACITY);

        for chr in line.chars() {
            sub_data.push(chr.to_digit(10).unwrap())
        }
        data.push(sub_data)
    }

    Grid::new(data)
}
