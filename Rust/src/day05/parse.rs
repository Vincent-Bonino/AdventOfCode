use nom::bytes::complete::tag;
use nom::character::complete::i32 as i32_parser;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};
use nom::{IResult, Parser};
use std::num::ParseIntError;

pub fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules: Vec<(i32, i32)> = Vec::with_capacity(1200);
    let mut updates: Vec<Vec<i32>> = Vec::with_capacity(300);

    let mut found_empty_line: bool = false;

    for line in input.lines() {
        if line.is_empty() {
            found_empty_line = true;
            continue;
        }

        // Regular parsing
        if !found_empty_line {
            let (_, (left, right)) = parse_rule(line).unwrap();
            rules.push((left, right));
        } else {
            let (_, update) = parse_update(line).unwrap();
            updates.push(update);
        }
    }

    (rules, updates)
}

fn parse_rule(rule_line: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32_parser, tag("|"), i32_parser).parse(rule_line)
}

fn parse_update(update_line: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(tag(","), i32_parser).parse(update_line)
}
