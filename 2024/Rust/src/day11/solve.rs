use rayon::prelude::*;
use std::collections::HashMap;

use crate::day11::model::Stone;

const BLINK_NUMBER_P1: usize = 25;
const BLINK_NUMBER_P2: usize = 75;

pub fn solve_part_one(stones: &[Stone]) -> i128 {
    count_stones(stones, BLINK_NUMBER_P1)
}

pub fn solve_part_two(stones: &[Stone]) -> i128 {
    count_stones(stones, BLINK_NUMBER_P2)
}

/// Count stones after `blink_number` blinks.
///
/// Uses the [`Stone.multiplier`] attributes to simplify the set of stone after each iteration
/// by keeping only one Stone of each type.
///
/// Further optimization could probably be done, caching evolution at different iterations.
/// The two parts run in ~1/4 sec, so it is not mandatory.
fn count_stones(stones: &[Stone], blink_number: usize) -> i128 {
    let mut local_stones: Vec<Stone> = stones.to_vec();

    for _ in 0..blink_number {
        let mut cache: HashMap<u128, Stone> = HashMap::new();

        // Compute a new round
        let new_stones: Vec<Stone> = local_stones
            .iter()
            .flat_map(|stone| stone.change())
            .collect();

        // Remove duplicates
        for stone in new_stones {
            cache
                .entry(stone.value)
                .and_modify(|entry| entry.multiplier += stone.multiplier)
                .or_insert(stone);
        }

        // Use deduplicated stones for the next iteration
        local_stones = cache.into_values().collect();
    }

    local_stones
        .iter()
        .map(|stone| stone.multiplier)
        .sum::<u128>() as i128
}
