use crate::day03::model::Instruction;
use nom::character::complete::i128;

pub fn solve_part_one(instructions: &[Instruction]) -> i128 {
    let result: i32 = instructions
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mul(left, right) => left * right,
            _ => 0,
        })
        .sum();
    result as i128
}

pub fn solve_part_two(instructions: &[Instruction]) -> i128 {
    let mut enabled: bool = true;
    let mut result: i32 = 0;

    for instr in instructions {
        match instr {
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Mul(left, right) => {
                if enabled {
                    result += left * right
                }
            }
        }
    }

    result as i128
}
