use nom::bytes::complete::tag;
use nom::character::complete::u64 as u64_parser;
use nom::combinator::map_res;
use nom::error::ErrorKind;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

use super::model::FreshRange;

pub fn parse_input(input: &str) -> (Vec<FreshRange>, Vec<u64>) {
    // Windows windows windows ...
    let (input_part1, input_part2): (&str, &str) = input.split_once("\r\n\r\n").unwrap();

    let fresh_ranges: Vec<FreshRange> = input_part1.lines().map(|l| parse_range(l).unwrap().1).collect();
    let ids: Vec<u64> = input_part2.lines().map(|l| l.parse::<u64>().unwrap()).collect();

    (fresh_ranges, ids)
}

pub fn parse_range(input: &str) -> IResult<&str, FreshRange> {
    map_res(separated_pair(u64_parser, tag("-"), u64_parser), |(lhs, rhs)| {
        Ok::<FreshRange, ErrorKind>(FreshRange { start: lhs, end: rhs })
    })
    .parse(input)
}
