use crate::day07::model::Equation;
use nom::bytes::complete::tag;
use nom::character::complete::{space1, u128 as u128_parser};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{IResult, Parser};
use std::num::ParseIntError;

pub fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}

fn parse_line(line: &str) -> IResult<&str, Equation> {
    map_res(
        tuple((u128_parser, tag(": "), separated_list1(space1, u128_parser))),
        |(target, _, values)| Ok::<_, ParseIntError>(Equation::new(target, values)),
    )
    .parse(line)
}
