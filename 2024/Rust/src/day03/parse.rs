use crate::day03::model::Instruction;
use regex::{Captures, Regex};

pub fn parse_input(input: &str) -> Vec<Instruction> {
    let instruction_regex: Regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mul_instruction_regex: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result: Vec<Instruction> = Vec::new();

    // Iter on all matches
    for mtc in instruction_regex.find_iter(input) {
        match mtc.as_str() {
            "don't()" => result.push(Instruction::Dont),
            "do()" => result.push(Instruction::Do),
            _ => result.push(parse_mul_instruction(&mul_instruction_regex, mtc.as_str())),
        }
    }

    result
}

fn parse_mul_instruction(mul_regex: &Regex, value: &str) -> Instruction {
    let captures: Captures = mul_regex.captures(value).unwrap();
    let left_val = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let right_val = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
    Instruction::Mul(left_val, right_val)
}
