use rayon::prelude::*;

use crate::day07::model::{Equation, Operator};

pub fn solve_part_one(equations: &[Equation]) -> i128 {
    let operators: Vec<Operator> = vec![Operator::Add, Operator::Multiply];

    let result: u128 = equations
        .par_iter()
        .filter(|equation| equation.is_resolvable(&operators))
        .map(|equation| equation.target)
        .sum();
    result as i128
}

pub fn solve_part_two(equations: &[Equation]) -> i128 {
    let operators: Vec<Operator> = vec![Operator::Add, Operator::Multiply, Operator::Concatenate];

    let result: u128 = equations
        .par_iter()
        .filter(|equation| equation.is_resolvable(&operators))
        .map(|equation| equation.target)
        .sum();
    result as i128
}
