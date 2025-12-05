use itertools::Itertools;

use super::model::FreshRange;

pub fn merge_ranges(ranges: &[FreshRange]) -> Vec<FreshRange> {
    let mut result: Vec<FreshRange> = Vec::with_capacity(ranges.len());

    let sorted_ranges: Vec<FreshRange> = ranges.iter().cloned().sorted().collect();
    let mut current_range = sorted_ranges[0];

    for r in sorted_ranges.into_iter().skip(1) {
        if !current_range.try_merge_with(&r) {
            result.push(current_range);
            current_range = r;
        }
    }

    // Do not forget to push the last one
    result.push(current_range);

    result
}

/// Assumes the ranges have already been merged.
pub fn solve_part_one(fresh_ranges: &[FreshRange], ids: &[u64]) -> String {
    ids.iter()
        .filter(|id| fresh_ranges.iter().any(|range| range.contains(**id)))
        .count()
        .to_string()
}

/// Assumes the ranges have already been merged.
pub fn solve_part_two(fresh_ranges: &[FreshRange]) -> String {
    fresh_ranges.iter().map(|range| range.len()).sum::<u64>().to_string()
}
