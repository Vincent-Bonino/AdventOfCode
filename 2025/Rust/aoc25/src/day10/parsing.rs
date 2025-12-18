use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, one_of, u64 as u64_parser};
use nom::combinator::map_res;
use nom::error::ErrorKind;
use nom::multi::{many1, separated_list1};
use nom::sequence::delimited;
use nom::{IResult, Parser};

use super::model::{Button, InputLine, IntButton, PartTwoTarget};

pub fn parse_input(input: &str) -> Vec<InputLine> {
    let (_, result) = separated_list1(line_ending, parse_line).parse(input).unwrap();
    result
}

fn parse_line(line: &str) -> IResult<&str, InputLine> {
    map_res(
        (
            parse_part_one_target,
            tag(" "),
            parse_buttons,
            tag(" "),
            parse_part_two_target,
        ),
        |(a, _, c, _, e)| Ok::<_, ErrorKind>((a, c, e)),
    )
    .parse(line)
}

fn parse_part_one_target(input: &str) -> IResult<&str, IntButton> {
    map_res(
        delimited(tag("["), many1(one_of(".#")), tag("]")),
        |values: Vec<char>| {
            Ok::<IntButton, ErrorKind>(values.into_iter().enumerate().fold(0_u64, |acc, (index, elem)| {
                if elem == '.' {
                    acc
                } else if elem == '#' {
                    acc + 2_u64.pow(index as u32)
                } else {
                    unreachable!("Unexpected char '{elem}")
                }
            }))
        },
    )
    .parse(input)
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<Button>> {
    separated_list1(tag(" "), parse_one_button).parse(input)
}

fn parse_one_button(input: &str) -> IResult<&str, Button> {
    map_res(
        delimited(tag("("), separated_list1(tag(","), u64_parser), tag(")")),
        |raw| Ok::<_, ErrorKind>(Button::new(raw)),
    )
    .parse(input)
}

fn parse_part_two_target(input: &str) -> IResult<&str, PartTwoTarget> {
    delimited(tag("{"), separated_list1(tag(","), u64_parser), tag("}")).parse(input)
}
