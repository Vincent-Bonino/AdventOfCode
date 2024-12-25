use crate::day25::model::{Key, Lock, MAX_LENGTH};

pub fn solve_part_one(keys: &Vec<Key>, locks: &Vec<Lock>) -> i64 {
    let mut result: i64 = 0;

    for key in keys.iter() {
        'lock_loop: for lock in locks.iter() {
            for (key_pin, lock_pin) in key.iter().zip(lock) {
                if key_pin + lock_pin > MAX_LENGTH {
                    continue 'lock_loop;  // This key does not fit in this lock
                }
            }

            // Key fits in the lock
            result += 1;
        }
    }


    result
}
