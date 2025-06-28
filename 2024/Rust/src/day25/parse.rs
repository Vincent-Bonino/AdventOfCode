use crate::day25::model::{Key, Lock};

pub fn parse_input(input: &str) -> (Vec<Key>, Vec<Lock>) {
    let mut keys: Vec<Key> = Vec::new();
    let mut locks: Vec<Lock> = Vec::new();

    for input_part in input.split("\r\n\r\n") { // Windows windows windows ...
        let (is_lock, key_or_lock) = parse_key_and_lock(input_part);

        if is_lock {
            locks.push(key_or_lock)
        } else {
            keys.push(key_or_lock)
        }
    }

    (keys, locks)
}

fn parse_key_and_lock(lines: &str) -> (bool, [u8; 5]) {
    let mut is_lock: Option<bool> = None;
    let mut element: [u8; 5] = [0; 5];

    let line_nb: usize = lines.lines().count();
    for (line_index, line) in lines.lines().enumerate() {
        // Depending on the first line, determine if the input is a key or a lock
        if line_index == 0 {
            is_lock = Some(line == "#####");
            continue; // Skip first line
        }
        if line_index == line_nb - 1 {
            continue; // Skip last line
        }

        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                element[i] += 1
            }
        }
    }

    (is_lock.unwrap(), element)
}
