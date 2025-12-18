use hashbrown::HashMap;
// use indexmap::IndexMap;
use itertools::Itertools;

use super::model::{Button, InputLine, IntButton, PartTwoTarget};

// Part one

pub fn solve_part_one(lines: &[InputLine]) -> String {
    let mut result: usize = 0;

    for (target, buttons, _) in lines.iter() {
        result += init_machine_p1(buttons, target);
    }

    result.to_string()
}

fn init_machine_p1(buttons: &[Button], target: &IntButton) -> usize {
    for i in 1..buttons.len() {
        for comb in buttons.iter().combinations(i) {
            let res = comb.iter().map(|btn| btn.to_int()).fold(0, |acc, val| acc ^ val);
            if res == *target {
                return comb.len();
            }
        }
    }
    unreachable!("Unable to solve part one: {:?} -> {}", buttons, target);
}

// Part two

pub fn solve_part_two(lines: &[InputLine]) -> String {
    let mut result: usize = 0;

    for (_, buttons, target) in lines.iter() {
        result += init_machine_p2(buttons, target);
    }

    result.to_string()
}

fn init_machine_p2(buttons: &[Button], target: &PartTwoTarget) -> usize {
    let all_presses: Vec<Vec<&Button>> = (1..=buttons.len())
        .flat_map(|i| buttons.iter().combinations(i))
        .collect();

    let mut cache: HashMap<PartTwoTarget, Option<usize>> = HashMap::new();
    _init_machine_p2(&all_presses, target, &mut cache).unwrap()

    // -- Debug, show pressed buttons
    // let mut cache: HashMap<PartTwoTarget, Option<Vec<&Button>>> = HashMap::new();
    // let pressed_buttons: Vec<&Button> = _init_machine_p2_buttons(&all_presses, target, 0, &mut cache).unwrap();
    // let result: usize = pressed_buttons.len();
    //
    // let mut pressed: IndexMap<&Button, usize> = IndexMap::new();
    // for btn in buttons {
    //     pressed.insert(btn, 0);
    // }
    // for btn in pressed_buttons {
    //     pressed.entry(btn).and_modify(|v| *v += 1).or_insert(1);
    // }
    //
    // // println!("\nTo solve {target:?}");
    // println!("-> {}", pressed.values().sum::<usize>());
    // println!("{:#?}", pressed);
    //
    // if !verify_solution(&pressed, target) {
    //     println!("Not verified");
    // }
    //
    // result
}

fn _init_machine_p2(
    presses: &[Vec<&Button>],
    target: &PartTwoTarget,
    cache: &mut HashMap<PartTwoTarget, Option<usize>>,
) -> Option<usize> {
    // Exit conditions
    if cache.contains_key(target) {
        return cache[target];
    }
    if target.iter().all(|t| *t == 0) {
        cache.insert((*target).clone(), Some(0));
        return Some(0);
    }

    //
    // Sub-computations
    //

    // A. Compute partial solutions, i.e. solving to have an all-even target, then solve the half of it
    let partial_int_target: IntButton = p2_target_to_int_button(target);
    let partial_solutions: Vec<&Vec<&Button>> = init_partial_machine_p2(presses, partial_int_target);

    let mut sub_results: Vec<usize> = Vec::with_capacity(partial_solutions.len() + 1);

    for part_sol in partial_solutions {
        // Make sure this partial solution is possible (not having a negative value for instance)
        if let Some(partial_target) = p2_apply_presses_to_target(part_sol, target) {
            let halved_target: PartTwoTarget = p2_target_to_halved(partial_target);

            if let Some(mut halved_solution) = _init_machine_p2(presses, &halved_target, cache) {
                sub_results.push(part_sol.len() + 2 * halved_solution);
            }
        }
    }

    // B. For already all-even targets, the computation is simpler
    if target.iter().all(|t| *t % 2 == 0) {
        let result = _init_machine_p2(presses, &p2_target_to_halved((*target).clone()), cache).map(|r| 2 * r);

        if let Some(res) = result {
            cache.insert((*target).clone(), Some(res));
            sub_results.push(res);
        }
    }

    //
    // Cache and return
    //

    let result = sub_results.iter().min().copied();
    cache.insert((*target).clone(), result);
    result
}

fn init_partial_machine_p2<'b>(presses: &'b [Vec<&'b Button>], target: IntButton) -> Vec<&'b Vec<&'b Button>> {
    let mut result: Vec<&Vec<&Button>> = Vec::new();

    for comb in presses.iter() {
        let res = comb.iter().map(|btn| btn.to_int()).fold(0, |acc, val| acc ^ val);
        if res == target {
            result.push(comb);
        }
    }

    result
}

#[inline]
fn p2_target_to_int_button(target: &PartTwoTarget) -> IntButton {
    target
        .iter()
        .enumerate()
        .filter_map(|(i, t)| if *t % 2 == 0 { None } else { Some(2_u64.pow(i as u32)) })
        .sum()
}

#[inline]
fn p2_target_to_halved(target: PartTwoTarget) -> PartTwoTarget {
    target.into_iter().map(|t| (t / 2)).collect()
}

#[inline]
fn p2_apply_presses_to_target(presses: &[&Button], target: &PartTwoTarget) -> Option<PartTwoTarget> {
    let mut result: PartTwoTarget = target.clone();

    for idx in presses.iter().flat_map(|btn| btn.raw()) {
        let idx = *idx as usize;
        match result[idx].checked_sub(1) {
            None => return None,
            Some(new) => result[idx] = new,
        }
    }

    Some(result)
}

// --- Debug ---

// /// Debug/visualization version, working of vecs of buttons.
// fn _init_machine_p2_buttons<'b>(
//     presses: &'b [Vec<&'b Button>],
//     target: &PartTwoTarget,
//     depth: usize,
//     cache: &mut HashMap<PartTwoTarget, Option<Vec<&'b Button>>>,
// ) -> Option<Vec<&'b Button>> {
//     // Exit conditions
//     if cache.contains_key(target) {
//         return cache[target].clone();
//     }
//     if target.iter().all(|t| *t == 0) {
//         cache.insert((*target).clone(), Some(Vec::new()));
//         return Some(Vec::new());
//     }
//
//     //
//     // Sub-computations
//     //
//
//     // A. Compute partial solutions, i.e. solving to have an all-even target, then solve the half of it
//     let partial_int_target: IntButton = p2_target_to_int_button(target);
//     let partial_solutions: Vec<&Vec<&Button>> = init_partial_machine_p2(presses, partial_int_target);
//
//     let mut sub_results: Vec<Vec<&Button>> = Vec::with_capacity(partial_solutions.len() + 1);
//
//     for part_sol in partial_solutions {
//         // Make sure this partial solution is possible (not having a negative value for instance)
//         if let Some(partial_target) = p2_apply_presses_to_target(part_sol, target) {
//             let halved_target: PartTwoTarget = p2_target_to_halved(partial_target);
//
//             if let Some(mut halved_solution) = _init_machine_p2_buttons(presses, &halved_target, depth + 1, cache) {
//                 // result = 2 * halved_result + partial_result
//                 halved_solution.extend(halved_solution.clone());
//                 halved_solution.extend(part_sol);
//
//                 sub_results.push(halved_solution);
//             }
//         }
//     }
//
//     // B. For already all-even targets, the computation is simpler
//     if target.iter().all(|t| *t % 2 == 0) {
//         let result =
//             _init_machine_p2_buttons(presses, &p2_target_to_halved(target.clone()), depth + 1, cache).map(|mut res| {
//                 res.extend(res.clone());
//                 res
//             });
//
//         if let Some(res) = result {
//             cache.insert((*target).clone(), Some(res.clone()));
//             sub_results.push(res);
//         }
//     }
//
//     //
//     // Cache and return
//     //
//
//     sub_results.sort_by_key(|a| a.len());
//
//     let result = sub_results.first().cloned();
//     cache.insert((*target).clone(), result.clone());
//     result
// }

// fn press_button(button: &Button, mut target: PartTwoTarget) -> PartTwoTarget {
//     for btn in button.raw() {
//         target[*btn as usize] += 1;
//     }
//     target
// }

// fn verify_solution(solution: &IndexMap<&Button, usize>, target: &PartTwoTarget) -> bool {
//     let mut current = vec![0; target.len()];
//
//     for (&btn, presses) in solution {
//         for _ in 0..(*presses) {
//             current = press_button(btn, current);
//         }
//     }
//
//     current == *target
// }
