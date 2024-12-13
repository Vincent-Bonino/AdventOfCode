use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, u64 as u64_parser};
use nom::combinator::map_res;
use nom::complete::take;
use nom::error::ErrorKind;
use nom::sequence::{separated_pair, tuple};
use nom::{IResult, Parser};

use crate::day13::model::ClawMachine;

pub fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut machines: Vec<ClawMachine> = Vec::with_capacity(input.len() / 4);

    for mut chunk in &input.lines().filter(|line| !line.is_empty()).chunks(3) {
        let line_a: &str = chunk.next().expect("Chunk error, no line 0");
        let line_b: &str = chunk.next().expect("Chunk error, no line 1");
        let line_p: &str = chunk.next().expect("Chunk error, no line 2");
        machines.push(parse_claw_machine(line_a, line_b, line_p))
    }

    machines
}

fn parse_claw_machine(line_a: &str, line_b: &str, line_p: &str) -> ClawMachine {
    let (_, button_a) = parse_button(line_a).expect("Error on button A");
    let (_, button_b) = parse_button(line_b).expect("Error on button B");
    let (_, prize) = parse_prize(line_p).expect("Error on prize");

    ClawMachine::new(button_a, button_b, prize)
}

fn parse_button(line: &str) -> IResult<&str, (u64, u64)> {
    map_res(
        tuple((
            alt((tag("Button A: "), tag("Button B: "))),
            parse_coordinates,
        )),
        |(_, val)| Ok::<_, ErrorKind>(val),
    )(line)
}

fn parse_prize(line: &str) -> IResult<&str, (u64, u64)> {
    map_res(tuple((tag("Prize: "), parse_coordinates)), |(_, val)| {
        Ok::<_, ErrorKind>(val)
    })(line)
}

fn parse_coordinates(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(parse_x_or_y, tag(", "), parse_x_or_y)(input)
}

fn parse_x_or_y(input: &str) -> IResult<&str, u64> {
    map_res(
        tuple((
            alt((tag("X"), tag("Y"))),
            alt((tag("+"), tag("="))),
            u64_parser,
        )),
        |(_, _, val)| Ok::<_, ErrorKind>(val),
    )(input)
}
