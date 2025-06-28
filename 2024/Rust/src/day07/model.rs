use crate::toolbox::power;
use itertools::Itertools;

pub struct Equation {
    pub target: u128,
    values: Vec<u128>,
}

#[derive(Copy, Clone, Debug)]
pub enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Equation {
    pub fn new(target: u128, values: Vec<u128>) -> Self {
        Equation { target, values }
    }

    fn solve(&self, operators: &[Operator]) -> bool {
        if self.values.len() - 1 != operators.len() {
            panic!("Wrong length of operators")
        }

        let mut result: u128 = *self.values.first().unwrap();

        for (index, op) in operators.iter().enumerate() {
            match op {
                Operator::Add => result += self.values.get(index + 1).unwrap(),
                Operator::Multiply => result *= self.values.get(index + 1).unwrap(),
                Operator::Concatenate => {
                    result = concat(&result, self.values.get(index + 1).unwrap())
                }
            }
        }

        result == self.target
    }

    /// Determine if a solution exists
    pub fn is_resolvable(&self, operators: &[Operator]) -> bool {
        // Brute force all the combinations
        let operator_combinations: Vec<Vec<Operator>> = power(operators, self.values.len() - 1);
        operator_combinations.iter().any(|ops| self.solve(ops))
    }
}

#[inline]
fn concat(left: &u128, right: &u128) -> u128 {
    left * 10_u128.pow(right.ilog10() + 1) + right
}
