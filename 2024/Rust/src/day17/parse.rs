use crate::day17::model::Computer;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{u32 as u32_parser, u8 as u8_parser};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::io::ErrorKind;

pub fn parse_input(input: &str) -> Computer {
    let lines: Vec<&str> = input.lines().collect();

    let (_, reg_a) = parse_registry(lines[0]).unwrap();
    let (_, reg_b) = parse_registry(lines[1]).unwrap();
    let (_, reg_c) = parse_registry(lines[2]).unwrap();
    // Empty line
    let (_, program) = parse_program(lines[4]).unwrap();

    Computer::new(reg_a, reg_b, reg_c, program)
}

fn parse_registry(registry_line: &str) -> IResult<&str, u32> {
    map_res(
        tuple((
            alt((
                tag("Register A: "),
                tag("Register B: "),
                tag("Register C: "),
            )),
            u32_parser,
        )),
        |(_register, value)| Ok::<_, ErrorKind>(value),
    )(registry_line)
}

fn parse_program(program_line: &str) -> IResult<&str, Vec<u8>> {
    map_res(
        tuple((tag("Program: "), separated_list1(tag(","), u8_parser))),
        |(_program, value)| Ok::<_, ErrorKind>(value),
    )(program_line)
}
