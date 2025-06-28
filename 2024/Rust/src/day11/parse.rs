use crate::day11::model::Stone;
use nom::character::complete::{space1, u128 as u128_parser};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::Parser;
use std::num::ParseIntError;

pub fn parse_input(input: &str) -> Vec<Stone> {
    input
        .trim()
        .split(' ')
        .map(|str_val: &str| Stone::new(str_val.parse::<u128>().unwrap()))
        .collect()
}
