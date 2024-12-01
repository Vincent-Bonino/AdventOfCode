use std::collections::HashMap;

pub fn solve_part_one(list1: &[i32], list2: &[i32]) -> i128 {
    let mut left_list: Vec<i32> = list1.to_owned();
    let mut right_list: Vec<i32> = list2.to_owned();

    if left_list.len() != right_list.len() {
        panic!("The two lists should have the same length");
    }

    // Sort the list
    let length: usize = left_list.len(); // Both list have the same length
    left_list.sort();
    right_list.sort();

    let mut result: i128 = 0;
    for index in 0..length {
        result += (left_list.get(index).unwrap() - right_list.get(index).unwrap()).abs() as i128;
    }

    result
}

pub fn solve_part_two(list1: &[i32], list2: &[i32]) -> i128 {
    let mut right_counts: HashMap<i32, i32> = HashMap::new(); // Key = right list elements, value = counter
    for elmt in list2.iter() {
        let new_counter: i32 = match right_counts.get(elmt) {
            None => 1,
            Some(val) => val + 1,
        };
        right_counts.insert(*elmt, new_counter);
    }

    list1
        .iter()
        .map(|left_elmt: &i32| (*left_elmt * *right_counts.get(left_elmt).unwrap_or(&0)) as i128)
        .sum::<i128>()
}
