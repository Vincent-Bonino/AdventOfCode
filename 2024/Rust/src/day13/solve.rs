use rayon::prelude::*;

use crate::day13::model::ClawMachine;

pub fn solve_part_one(machines: &[ClawMachine]) -> u64 {
    machines
        .par_iter()
        .map(|machine| machine.get_token_price())
        .sum()
}

pub fn solve_part_two(machines: &[ClawMachine]) -> u64 {
    machines
        .to_vec()
        .iter_mut()
        .map(|machine| {
            machine.correct_prize_distance();
            machine.get_token_price()
        })
        .sum()
}
