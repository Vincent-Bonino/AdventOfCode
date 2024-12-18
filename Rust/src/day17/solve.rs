use rayon::prelude::*;
use std::collections::VecDeque;

use crate::day17::model::Computer;

pub fn solve_part_one(computer: &Computer) -> i128 {
    let mut computer: Computer = computer.clone();

    // Run the program
    let stdout: Vec<u8> = computer.run();
    let stdout_str: String = Computer::format_stdout(&stdout);

    println!("Computer's output is:");
    println!("{stdout_str}\n");

    // Return dummy value
    1234
}

pub fn solve_part_two(computer: &Computer) -> i128 {
    let reference_program: &Vec<u8> = &computer.stack;
    let mut result: Vec<u8> = Vec::new();

    // for i in 0..reference_program.len() {
    //     let i: u32 = i as u32;
    //     let program_mapping: Vec<u8> = vec![
    //         *run_with_a(computer, 0 * i).first().unwrap(),
    //         *run_with_a(computer, 1 * i).first().unwrap(),
    //         *run_with_a(computer, 2 * i).first().unwrap(),
    //         *run_with_a(computer, 3 * i).first().unwrap(),
    //         *run_with_a(computer, 4 * i).first().unwrap(),
    //         *run_with_a(computer, 5 * i).first().unwrap(),
    //         *run_with_a(computer, 6 * i).first().unwrap(),
    //         *run_with_a(computer, 7 * i).first().unwrap(),
    //     ];
    //
    //
    // }
    //

    for i in 0..7 {
        let i = i << 15;
        println!(
            "{i:3>0} (={}) => {}",
            Computer::format_stdout(&decompose(i)),
            Computer::format_stdout(&run_with_a(computer, i)),
        );
    }

    -1
}

fn run_with_a(computer: &Computer, a: u32) -> Vec<u8> {
    computer.with_a(a).run()
}

fn decompose(value: u32) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    let mut power: u32 = 8;
    while power != 0 {
        let new = (value >> (3 * power)) & 7;
        res.push(new as u8);

        power -= 1;
    }

    res
}
