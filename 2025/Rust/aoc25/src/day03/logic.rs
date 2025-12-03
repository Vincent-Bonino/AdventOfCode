//! Global idea:
//!  - it is *always* worth dumping everything after the current value if it increases it.
//!
//! Bank:       1 2 3 4 9 1 2 3
//! Current:    1 2 3               (123)
//!
//! Inspecting: 1 2 3 4 9 1 2 3
//!                   ^
//! Testing all our current values, from left to right, against the inspected value.
//! Here 4 > 1. It is *always* worth updating it, even if it would mean dropping better values
//! for other current value on the right.
//!
//! And so forth progressing rightward in the current value.
//!
//! Implementation rely on the fact that '0' < '1' < ... < '9'.

pub fn solve(battery_banks: &[Vec<char>], battery_count: usize) -> String {
    let mut result: u64 = 0;

    for bank in battery_banks {
        let mut selected_ind: Vec<usize> = vec![0; battery_count];
        let mut selected_val: Vec<char> = vec!['0'; battery_count];

        initialize(&mut selected_ind, &mut selected_val, 0, bank, 0);

        for pointer in 1..bank.len() {
            test_update(&mut selected_ind, &mut selected_val, bank, pointer);
        }

        result += to_decimal(&selected_val);
    }

    result.to_string()
}

// Utils

/// Initialize or update the selected values.
///
/// # Parameters
/// `selected_ind`  Array of currently selected indexes in the battery bank.
/// `selected_val`  Array of currently selected values in the battery bank.
/// `select_start`  Index at which to start the select initialization/update.
/// `bank`          Battery bank being processed.
/// `start_bank`    Index at which to start to read the bank.
fn initialize(
    selected_ind: &mut [usize],
    selected_val: &mut [char],
    select_start: usize,
    bank: &[char],
    start_bank: usize,
) {
    for i in select_start..selected_val.len() {
        let new_ind: usize = start_bank + i - select_start;
        selected_val[i] = bank[new_ind];
        selected_ind[i] = new_ind;
    }
}

/// Attempt to increate the "joltage" of the battery bank.
///
/// # Parameters
/// `selected_ind`  Array of currently selected indexes in the battery bank.
/// `selected_val`  Array of currently selected values in the battery bank.
/// `bank`          Battery bank being processed.
/// `pointer`       Index at which the bank is being inspected.
fn test_update(selected_ind: &mut [usize], selected_val: &mut [char], bank: &[char], pointer: usize) {
    let new_value: char = bank[pointer];

    // Do not attempt to over-read the bank
    let selected_start = selected_val.len().saturating_sub(bank.len() - pointer);

    for i in selected_start..selected_val.len() {
        if new_value > selected_val[i] && pointer > selected_ind[i] {
            initialize(selected_ind, selected_val, i, bank, pointer);
            break;
        }
    }
}

fn to_decimal(selected: &[char]) -> u64 {
    selected
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let value: u64 = (*value as u64) - 48;
            value * 10_u64.pow((selected.len() - index - 1) as u32)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chars() {
        assert_eq!('0' as u64, 48);
        assert_eq!('1' as u64, 49);
        // ...
        assert_eq!('9' as u64, 57);
    }

    #[test]
    fn test_to_decimal() {
        assert_eq!(to_decimal(&['0', '1']), 1);
        assert_eq!(to_decimal(&['1', '0']), 10);
        assert_eq!(to_decimal(&['4', '0', '9']), 409);
        assert_eq!(
            to_decimal(&['9', '8', '7', '6', '5', '4', '3', '2', '1', '0', '1', '2', '3']),
            9876543210123, // u32 is not enough!
        );
    }
}
