use nom::character::complete::{i32 as i32_parser, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{IResult, Parser};
use std::num::ParseIntError;

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::with_capacity(1000);

    for line in input.lines() {
        let (_, report): (_, Vec<i32>) = parse_line(line).unwrap();
        reports.push(report)
    }

    reports
}

fn parse_line(line: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, i32_parser).parse(line)
}
