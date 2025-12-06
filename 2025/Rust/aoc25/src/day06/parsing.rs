use nom::character::complete::{line_ending, multispace1, newline, one_of, space1, u64 as u64_parser};
use nom::combinator::{all_consuming, map_res, opt};
use nom::error::ErrorKind;
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;
use nom::{IResult, Parser};

use super::model::Problem;

pub fn parse_input(input: &str) -> (Vec<String>, Vec<char>) {
    let numbers_line_count: usize = input.lines().count() - 1;

    input.lines().enumerate().fold(
        (Vec::<String>::with_capacity(numbers_line_count), Vec::<char>::new()),
        |(mut acc_str, mut acc_char), (index, line)| {
            if index < numbers_line_count {
                acc_str.push(line.to_owned());
            } else {
                acc_char = parse_operators(line).unwrap().1;
            }
            (acc_str, acc_char)
        },
    )
}

#[rustfmt::skip]
pub fn parse_numbers_line(input: &str) -> IResult<&str, Vec<u64>> {
    map_res(
        (
            opt(space1),
            separated_list1(space1, u64_parser),
        ),
        |(_, numbers)| { Ok::<Vec<u64>, ErrorKind>(numbers)},
    ).parse(input)
}

fn parse_operators(input: &str) -> IResult<&str, Vec<char>> {
    separated_list1(many1(space1), one_of("+*")).parse(input)
}
