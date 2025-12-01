use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{i32 as i32_parser, one_of};
use nom::combinator::map_res;
use nom::error::ErrorKind;
use nom::{IResult, Parser};

pub fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|l| parse_rotation(l).unwrap().1).collect()
}

fn parse_rotation(line: &str) -> IResult<&str, i32> {
    map_res((alt((tag("L"), tag("R"))), i32_parser), |(dir, val)| match dir {
        "L" => Ok::<i32, ErrorKind>(-val),
        "R" => Ok::<i32, ErrorKind>(val),
        _ => unimplemented!("Unknown direction {dir}"),
    })
    .parse(line)
}
