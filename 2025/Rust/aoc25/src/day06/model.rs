#[derive(Debug)]
pub struct Problem {
    pub numbers: Vec<u64>,
    pub operator: char,
}

impl Problem {
    pub fn solve(&self) -> u64 {
        let (operation, init): (fn(u64, u64) -> u64, u64) = match self.operator {
            '+' => (::std::ops::Add::add, 0),
            '*' => (::std::ops::Mul::mul, 1),
            _ => unreachable!("Unknown operator {}", self.operator),
        };

        self.numbers.iter().fold(init, |acc, n| operation(acc, *n))
    }
}
