use crate::toolbox::Coordinates;
use nom::bytes::complete::tag;
use nom::character::complete::i32 as i32_parser;
use nom::combinator::map_res;
use nom::sequence::separated_pair;
use nom::IResult;
use std::io::ErrorKind;

pub fn parse_input(input: &str) -> Vec<Coordinates> {
    input
        .lines()
        .map(|line| {
            let (_, coord) = parse_coordinates(line).unwrap();
            coord
        })
        .collect()
}

fn parse_coordinates(line: &str) -> IResult<&str, Coordinates> {
    map_res(
        separated_pair(i32_parser, tag(","), i32_parser),
        |(x, y)| Ok::<_, ErrorKind>(Coordinates { y, x }),
    )(line)
}
