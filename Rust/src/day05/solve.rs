use std::collections::HashMap;

pub fn solve_part_one(rules: &[(i32, i32)], updates: &[Vec<i32>]) -> i128 {
    let mut result: i32 = 0;

    for update in updates {
        let (is_valid, _): (bool, _) = is_update_valid(update, rules);

        // If all rules are verified, add the middle element to the result
        if is_valid {
            result += update.get(update.len() / 2).unwrap();
        }
    }

    result as i128
}

fn is_update_valid(update: &[i32], rules: &[(i32, i32)]) -> (bool, Option<(usize, usize)>) {
    // Build a value-index hashmap
    let mut index_map: HashMap<i32, usize> = HashMap::with_capacity(update.len());
    for (index, value) in update.iter().enumerate() {
        index_map.insert(*value, index);
    }

    // Verify each rule
    for (before, after) in rules {
        if let Some(before_index) = index_map.get(before) {
            if let Some(after_index) = index_map.get(after) {
                if after_index < before_index {
                    // Update is invalid
                    return (false, Some((*before_index, *after_index)));
                }
            }
        }
    }

    // Update is valid
    (true, None)
}

pub fn solve_part_two(rules: &[(i32, i32)], updates: &mut [Vec<i32>]) -> i128 {
    let mut result: i32 = 0;

    for update in updates {
        let (is_valid, _): (bool, _) = is_update_valid(update, rules);

        // Take into account only incorrectly-ordered updates
        if is_valid {
            continue;
        }

        loop {
            let (is_valid, indexes): (bool, Option<(usize, usize)>) =
                is_update_valid(update, rules);

            if is_valid {
                break;
            } else {
                let (before, after): (usize, usize) = indexes.unwrap();
                update.swap(before, after)
            }
        }

        // Compute middle page
        result += update.get(update.len() / 2).unwrap();
    }

    result as i128
}
