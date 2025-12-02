use std::ops::Range;

use divisors_fixed::Divisors;
use hashbrown::HashMap;
use itertools::Itertools;

pub fn solve_part_one(ranges: &[(u64, u64)]) -> String {
    let mut result: u64 = 0;

    let mut divisors_cache: HashMap<u64, Vec<u64>> = HashMap::new();
    // Key is the length of the invalid IDs
    let mut invalid_ids_cache: HashMap<u64, Vec<u64>> = HashMap::new();

    for (min, max) in ranges.iter().copied() {
        let range = min..=max;

        let min_len: u64 = length_of(min);
        let max_len: u64 = length_of(max);

        // Compute cache if needed
        for len in min_len..=max_len {
            invalid_ids_cache
                .entry(len)
                .or_insert(invalid_ids_of_half_length(len, &mut divisors_cache));
        }

        // Read cache
        let mut all_invalid_by_length: Vec<&Vec<u64>> = Vec::with_capacity((max_len - min_len + 1) as usize);
        for len in min_len..=max_len {
            all_invalid_by_length.push(invalid_ids_cache.get(&len).unwrap());
        }

        let invalid_ids_in_range = all_invalid_by_length
            .into_iter()
            .flatten()
            .unique()
            .filter(|inv_id| range.contains(inv_id));

        result += invalid_ids_in_range.sum::<u64>();
    }

    result.to_string()
}

/// Compute all the invalid IDs of half the provided length.
fn invalid_ids_of_half_length(length: u64, divisors_cache: &mut HashMap<u64, Vec<u64>>) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();

    let divisors: &Vec<u64> = divisors_cache.entry(length).or_insert(length.divisors());

    for (pattern_count, pattern_len) in divisors.iter().zip(divisors.iter().rev()) {
        if *pattern_count != 2 {
            continue;
        } // Pattern must repeat exactly twice

        let bases: Range<u64> = generate_patterns_of_len(*pattern_len);
        result.extend(
            bases
                .into_iter()
                .map(|pattern| expand_pattern(pattern, *pattern_len, *pattern_count)),
        )
    }

    result
}

// Part two

pub fn solve_part_two(ranges: &[(u64, u64)]) -> String {
    let mut result: u64 = 0;

    let mut divisors_cache: HashMap<u64, Vec<u64>> = HashMap::new();
    // Key is the length of the invalid IDs
    let mut invalid_ids_cache: HashMap<u64, Vec<u64>> = HashMap::new();

    for (min, max) in ranges.iter().copied() {
        let range = min..=max;

        let min_len: u64 = length_of(min);
        let max_len: u64 = length_of(max);

        // Compute cache if needed
        for len in min_len..=max_len {
            invalid_ids_cache
                .entry(len)
                .or_insert(invalid_ids_of_length(len, &mut divisors_cache));
        }

        // Read cache
        let mut all_invalid_by_length: Vec<&Vec<u64>> = Vec::with_capacity((max_len - min_len + 1) as usize);
        for len in min_len..=max_len {
            all_invalid_by_length.push(invalid_ids_cache.get(&len).unwrap());
        }

        let invalid_ids_in_range = all_invalid_by_length
            .into_iter()
            .flatten()
            .unique()
            .filter(|inv_id| range.contains(inv_id));

        result += invalid_ids_in_range.sum::<u64>();
    }

    result.to_string()
}

/// Compute all the invalid IDs of the provided length.
fn invalid_ids_of_length(length: u64, divisors_cache: &mut HashMap<u64, Vec<u64>>) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();

    let divisors: &Vec<u64> = divisors_cache.entry(length).or_insert(length.divisors());

    for (pattern_count, pattern_len) in divisors.iter().zip(divisors.iter().rev()) {
        if *pattern_count == 1 {
            continue;
        } // Pattern must repeat at least once

        let bases: Range<u64> = generate_patterns_of_len(*pattern_len);
        result.extend(
            bases
                .into_iter()
                .map(|pattern| expand_pattern(pattern, *pattern_len, *pattern_count)),
        )
    }

    result
}

// Utils

#[inline]
/// Determine the length of an integer.
fn length_of(val: u64) -> u64 {
    (1 + val.ilog10()) as u64
}

#[inline]
/// Generate all the base patterns of provided length.
///
/// Note that patterns can not start with '0'.
fn generate_patterns_of_len(pattern_len: u64) -> Range<u64> {
    let min: u64 = 10_u64.pow(pattern_len as u32 - 1);
    let max: u64 = 10_u64.pow(pattern_len as u32);

    min..max
}

#[inline]
/// Expand the provided pattern `count` times.
fn expand_pattern(pattern: u64, pattern_len: u64, count: u64) -> u64 {
    let mut result: u64 = 0;

    for i in 0..count {
        result += pattern * 10_u64.pow((i * pattern_len) as u32);
    }

    result
}

// /// Naive part one solution.
// pub fn solve_part_one(ranges: &[(u64, u64)]) -> String {
//     let mut result: u64 = 0;
//
//     for (min, max) in ranges.iter().copied() {
//         for val in min..=max {
//             let half_len: u32 = val.ilog10().div_ceil(2);
//
//             let lhs = val / (10_u64.pow(half_len));
//             let rhs = val % (10_u64.pow(half_len));
//
//             if lhs == rhs {
//                 result += val;
//             }
//         }
//     }
//
//     result.to_string()
// }

#[cfg(test)]
mod test {
    use super::*;

    use divisors_fixed::Divisors;

    #[test]
    fn test_log() {
        assert_eq!(5_u64.ilog10(), 0);
        assert_eq!(10_u64.ilog10(), 1);
        assert_eq!(50_u64.ilog10(), 1);
        assert_eq!(100_u64.ilog10(), 2);
    }

    #[test]
    fn test_divisors() {
        assert_eq!(5_u64.divisors(), vec![1, 5]);
        assert_eq!(6_u64.divisors(), vec![1, 2, 3, 6]);
    }

    #[test]
    fn test_patterns() {
        assert_eq!(generate_patterns_of_len(1), 1..10);
        assert_eq!(generate_patterns_of_len(2), 10..100);
    }

    #[test]
    fn test_expand() {
        assert_eq!(expand_pattern(123, 3, 2), 123123);
        assert_eq!(expand_pattern(1, 1, 4), 1111);
    }

    #[test]
    fn test_invalid() {
        let mut cache = HashMap::new();
        assert_eq!(
            invalid_ids_of_length(2, &mut cache),
            vec![11, 22, 33, 44, 55, 66, 77, 88, 99]
        );
    }
}
