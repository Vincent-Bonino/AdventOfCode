use std::cmp;

use itertools::{all, Itertools};
use rayon::prelude::*;

use crate::day22::model::{Buyer, Monkey};

const DAY_LENGTH: usize = 2000;

pub fn solve_part_one(seeds: &[u64]) -> u64 {
    let mut buyers: Vec<Buyer> = seeds.iter().map(|seed| Buyer::new(*seed)).collect();

    buyers
        .iter_mut()
        .map(|buyer| buyer.get_secret_after(DAY_LENGTH))
        .sum()
}

/// Horrible solution, with an unnecessary bruteforce.
/// Runs in about 45 seconds (speed is a passion)
pub fn solve_part_two(seeds: &[u64]) -> u64 {
    let mut buyers: Vec<Buyer> = seeds.iter().map(|seed| Buyer::new(*seed)).collect();

    // For each buyer, the sequence of price they will offer for a hiding spot
    let prices: Vec<Vec<u64>> = buyers
        .iter_mut()
        .map(|buyer| {
            (0..=DAY_LENGTH)
                .map(|i| {
                    if i == 0 {
                        buyer.secret % 10
                    } else {
                        buyer.get_secret_after(1) % 10
                    }
                })
                .collect()
        })
        .collect();

    // For each buyer, the sequence of price variations
    let price_variations: Vec<Vec<i64>> = prices
        .iter()
        .map(|price_seq| {
            price_seq
                .iter()
                .tuple_windows()
                .map(|(prev, next)| *next as i64 - *prev as i64)
                .collect()
        })
        .collect();

    // Generate all possible buying instructions
    let all_buying_instructions: Vec<Vec<i64>> = get_buying_instructions_rec(vec![], 4);
    println!(
        "Total buying instructions: {}",
        all_buying_instructions.len()
    );

    let mut monkeys: Vec<Monkey> = all_buying_instructions
        .into_par_iter()
        .map(|instr| {
            let mut monkey: Monkey = Monkey::new(instr.try_into().expect("Vec to arr error"));

            for i in 0..prices.len() {
                monkey.try_to_buy_from(&prices[i], &price_variations[i]);
            }

            monkey
        })
        .collect();

    // Sort monkeys by number of bought bananas
    monkeys.sort_by_key(|monkey| monkey.collected_bananas);
    monkeys.last().unwrap().collected_bananas
}

fn get_buying_instructions_rec(partial_result: Vec<i64>, depth: usize) -> Vec<Vec<i64>> {
    if depth == 0 {
        return if are_instructions_valid(&partial_result) {
            vec![partial_result]
        } else {
            vec![]
        };
    }

    let mut result: Vec<Vec<i64>> = Vec::new();

    for i in -9..=9 {
        let mut new_partial_result: Vec<i64> = partial_result.clone();
        new_partial_result.push(i);

        if are_instructions_valid(&new_partial_result) {
            result.extend(get_buying_instructions_rec(new_partial_result, depth - 1))
        }
    }
    result
}

fn are_instructions_valid(instructions: &[i64]) -> bool {
    let min: i64 = -9; // Starting at 9 and only decreasing, down to 0
    let max: i64 = 9; // Starting at 0 and only increasing, up to 9

    let range = min..=max;

    let first_val: i64 = *instructions.first().unwrap();

    let mut current_min: i64 = first_val;
    let mut current_max: i64 = first_val;
    let mut sum: i64 = first_val;

    for val in instructions.iter().skip(1) {
        sum += val;
        current_min = cmp::min(current_min, sum);
        current_max = cmp::max(current_max, sum);

        if (current_max - current_min).abs() > 9 {
            // Impossible price variation
            return false;
        }

        if !range.contains(&sum) {
            // Impossible price variation
            return false;
        }
    }

    // No impossible price variation
    true
}
