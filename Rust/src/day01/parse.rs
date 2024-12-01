use nom::character::complete::{i32 as i32_parser, space1};
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::{IResult, Parser};
use std::num::ParseIntError;

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::with_capacity(1000);
    let mut list2: Vec<i32> = Vec::with_capacity(1000);

    for line in input.lines() {
        let (_, (a, b)): (_, (i32, i32)) = parse_line(line).unwrap();
        list1.push(a);
        list2.push(b)
    }

    (list1, list2)
}

fn parse_line(line: &str) -> IResult<&str, (i32, i32)> {
    map_res(
        tuple((i32_parser, space1, i32_parser)),
        |(int1, _, int2)| Ok::<_, ParseIntError>((int1, int2)),
    )
    .parse(line)
}
