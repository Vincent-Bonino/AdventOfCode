use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub fn count_possibilities<'a>(
    towels: &HashSet<String>,
    pattern: &'a str,
    cache: &mut HashMap<&'a str, i64>,
    min_towel_len: usize,
    max_towel_len: usize,
) -> i64 {
    // Use cache if possible
    if let Some(cached_result) = cache.get(pattern) {
        return *cached_result;
    }

    // Compute
    let min_len: usize = min_towel_len;
    let max_len: usize = min(pattern.len(), max_towel_len);

    // Recursion limit
    if pattern.is_empty() {
        return 1;
    } else if pattern.len() < min_len {
        cache.insert(pattern, 0);
        return 0;
    }

    let mut result: i64 = 0;
    for i in min_len..=max_len {
        let pre_string = &pattern[0..i];
        if towels.contains(pre_string) {
            result +=
                count_possibilities(towels, &pattern[i..], cache, min_towel_len, max_towel_len);
        }
    }

    cache.insert(pattern, result);
    result
}
