use crate::coord;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::u32 as u32_parser;
use nom::combinator::map_res;
use nom::error::ErrorKind;
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};
use nom::{IResult, Parser};

use crate::prelude::*;

pub fn parse_input(input: &str) -> Vec<Coordinates> {
    let (_, coordinates) = many1(parse_coordinates).parse(input).unwrap();
    coordinates
}

fn parse_coordinates(line: &str) -> IResult<&str, Coordinates> {
    map_res(
        terminated(separated_pair(u32_parser, tag(","), u32_parser), line_ending),
        |(x, y)| Ok::<Coordinates, ErrorKind>(coord!(x, y)),
    )
    .parse(line)
}
