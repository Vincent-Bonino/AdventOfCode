use crate::day08::model::Point;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::u32 as u32_parser;
use nom::combinator::map_res;
use nom::error::ErrorKind;
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};
use nom::{IResult, Parser};

pub fn parse_input(input: &str) -> Vec<Point> {
    let (_, points) = many1(parse_point).parse(input).unwrap();
    points
}

#[rustfmt::skip]
pub fn parse_point(input: &str) -> IResult<&str, Point> {
    map_res(
        terminated(
            separated_pair(
                u32_parser,
                tag(","),
                separated_pair(
                    u32_parser,
                    tag(","),
                    u32_parser,
                )
            ),
            line_ending,
        ),
        |(val1, (val2, val3))| {
            Ok::<Point, ErrorKind>(Point(val1, val2, val3))
        }
    ).parse(input)
}
