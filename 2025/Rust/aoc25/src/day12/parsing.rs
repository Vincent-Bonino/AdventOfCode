use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, one_of, usize as usize_parser};
use nom::combinator::map_res;
use nom::error::ErrorKind;
use nom::multi::{many, many1, separated_list1};
use nom::sequence::{separated_pair, terminated};
use nom::{IResult, Parser};

use super::model::{Present, TreeRegion};
use crate::prelude::Direction;

pub fn parse_input(input: &str) -> (Vec<Present>, Vec<TreeRegion>) {
    let (_, (presents, regions)) = separated_pair(
        separated_list1(line_ending, parse_present),
        line_ending,
        many1(parse_region),
    )
    .parse(input)
    .unwrap();
    (presents, regions)
}

fn parse_present(input: &str) -> IResult<&str, Present> {
    map_res(
        (
            terminated(terminated(usize_parser, tag(":")), line_ending),
            many(
                Present::SIZE,
                terminated(many(Present::SIZE, one_of(".#")), line_ending),
            ),
        ),
        |(id, lines): (usize, Vec<Vec<char>>)| {
            let shape = lines.into_iter().flatten().enumerate().collect();
            Ok::<Present, ErrorKind>(Present { index: id, shape })
        },
    )
    .parse(input)
}

fn parse_region(input: &str) -> IResult<&str, TreeRegion> {
    map_res(
        terminated(
            separated_pair(
                separated_pair(usize_parser, tag("x"), usize_parser),
                tag(": "),
                separated_list1(tag(" "), usize_parser),
            ),
            line_ending,
        ),
        |((height, width), present_quantity)| {
            Ok::<TreeRegion, ErrorKind>(TreeRegion {
                height,
                width,
                present_quantity,
            })
        },
    )
    .parse(input)
}
