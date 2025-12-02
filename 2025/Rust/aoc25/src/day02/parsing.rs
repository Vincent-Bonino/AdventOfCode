use nom::bytes::complete::tag;
use nom::character::complete::u64 as u64_parser;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

pub fn parse_input(input: &str) -> Vec<(u64, u64)> {
    parse_ranges(input).expect("Parsing error").1
}

#[rustfmt::skip]
fn parse_ranges(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(
        tag(","),
        separated_pair(
            u64_parser,
            tag("-"),
            u64_parser,
        ),
    ).parse(input)
}
