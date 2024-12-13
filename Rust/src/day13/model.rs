use std::cmp::{max, min};
use std::collections::HashSet;

use crate::toolbox::maths::solve_linear_diophantine_equation;

const A_BUTTON_PRICE: u64 = 3;
const B_BUTTON_PRICE: u64 = 1;

const PRIZE_ERROR: u64 = 10_000_000_000_000;

#[derive(Copy, Clone, Debug)]
pub struct ClawMachine {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

impl ClawMachine {
    pub fn new(button_a: (u64, u64), button_b: (u64, u64), prize: (u64, u64)) -> Self {
        Self {
            button_a,
            button_b,
            prize,
        }
    }

    pub fn correct_prize_distance(&mut self) {
        self.prize = (self.prize.0 + PRIZE_ERROR, self.prize.1 + PRIZE_ERROR);
    }

    pub fn price(nb_presses_a: &u64, nb_presses_b: &u64) -> u64 {
        A_BUTTON_PRICE * nb_presses_a + B_BUTTON_PRICE * nb_presses_b
    }

    /// Attempt to solve a [`Bézout identity`] to determine:
    ///  1. if the prize can be taken,
    ///  2. if so, what is the minimum token price to get it.
    ///
    /// [`Bézout identity`]: https://en.wikipedia.org/wiki/B%C3%A9zout%27s_identity
    pub fn get_minimal_token_price(&self) -> u64 {
        // On each axis: attempting to solve Ax+By=P
        // with:
        //  - A being the movement of the claw when pression button A
        //  - B being the movement of the claw when pression button B
        //  - P being the position of the prize
        //
        // Note that an equation like this has either 0 or an infinite number of solution.
        // Note also that we want to find the same x and y for both axis !

        if let (Some(sol_x), Some(sol_y)) = (self.solve_x(), self.solve_y()) {
            let found_solutions = sol_x.solutions(&sol_y);

            // At this point, I'm sure why I have too many solutions.
            // With a clear head, I could probably see why
            // (probably the maths of .solutions() are like this).
            //
            // Anyway, after all the maths, I do the simplest double check:
            // I verify that the found solutions are actually solutions of the initial problem...
            //
            // I know, I know...
            let mut verified_solutions: Vec<(u64, u64)> = Vec::with_capacity(found_solutions.len());

            // Re-verify solutions
            for sol in found_solutions {
                let final_x = self.button_a.0 * sol.0 + self.button_b.0 * sol.1;
                let final_y = self.button_a.1 * sol.0 + self.button_b.1 * sol.1;

                if (final_x, final_y) == self.prize {
                    verified_solutions.push(sol);
                }
            }

            verified_solutions
                .iter()
                .map(|(x, y)| ClawMachine::price(x, y))
                .min()
                .unwrap_or(0)
        } else {
            // Missing at least a solution on X or Y, the prize is unwinnable.
            0
        }
    }

    fn solve_x(&self) -> Option<Solutions> {
        // Gather the variables
        let a: i64 = self.button_a.0 as i64;
        let b: i64 = self.button_b.0 as i64;
        let c: i64 = self.prize.0 as i64;

        // Solve
        solve_linear_diophantine_equation(a, b, c)
            .map(|(a1, b1, x0, y0)| Solutions::new(a1, b1, x0, y0))
    }

    fn solve_y(&self) -> Option<Solutions> {
        // Gather the variables
        let a: i64 = self.button_a.1 as i64;
        let b: i64 = self.button_b.1 as i64;
        let c: i64 = self.prize.1 as i64;

        // Solve
        solve_linear_diophantine_equation(a, b, c)
            .map(|(a1, b1, x0, y0)| Solutions::new(a1, b1, x0, y0))
    }
}

/// Represent the infinite set of solutions of an equation:
///
/// ```txt
/// a1 * x + b1 * y = c1
/// ```
///
/// Fields
///  - a1, b1, c1: variables of the equation
///  - x0, y0: part of a specific solution (not related to X and Y of the problem)
///    They could be named "a0" and "b0" with respect to the global puzzle.
#[derive(Debug)]
pub struct Solutions {
    a1: i64,
    b1: i64,
    x0: i64,
    y0: i64,
}

impl Solutions {
    pub fn new(a1: i64, b1: i64, x0: i64, y0: i64) -> Self {
        Self { a1, b1, x0, y0 }
    }

    pub fn solutions(&self, other: &Self) -> Vec<(u64, u64)> {
        // Now we have two solutions, one on X and one on Y.
        // On X: (x0  + b1 *k , y0  - a1 *k ) for k  in Z.
        // On Y: (x'0 + b'1*k', y'0 - a'1*k') for k' in Z.

        // We want to find all matching solutions
        // i.e.:
        //  { x0  + b1 *k  =  x'0 + b'1*k
        //  { y0  - a1 *k  =  y'0 - a'1*k'

        // A linear system of equations !
        let a: Vec<Vec<i64>> = vec![vec![self.b1, -other.b1], vec![-self.a1, other.a1]];
        let b: Vec<i64> = vec![other.x0 - self.x0, other.y0 - self.y0];

        // Using Cramer's rule (https://en.wikipedia.org/wiki/Cramer's_rule)
        let d: i64 = (a[0][0] * a[1][1]) - (a[1][0] * a[0][1]);
        let dx: i64 = (b[0] * a[1][1]) - (b[1] * a[0][1]);
        let dy: i64 = (a[0][0] * b[0]) - (a[1][0] * b[1]);

        // We have the two solutions !
        let x: i64 = dx / d; // x is k  in the above description
        let y: i64 = dy / d; // y is k' in the above description

        // Build the solutions
        let self_xs = self.x0 + self.b1 * x;
        let self_ys = self.y0 - self.a1 * x;

        let other_xs = other.x0 + other.b1 * y;
        let other_ys = other.y0 - other.a1 * y;

        let mut solutions: Vec<(u64, u64)> = Vec::new();

        // Our solutions represent a number of time we will press the A & B buttons.
        // They must be positive!
        if self_xs >= 0 && self_ys >= 0 {
            solutions.push((self_xs as u64, self_ys as u64));
        }
        if other_xs >= 0 && other_ys >= 0 {
            solutions.push((other_xs as u64, other_ys as u64));
        }

        // I'm sure why, but here I have sometimes too many solutions.
        // With a clear head, I could probably see why
        // (probably the maths of here explain this).
        //
        // Solutions will be double-checked after to filter out false positives.
        solutions
    }

    // First approach, working but inefficient with large A, B and C.
    // Keeping it for the record.
    #[allow(dead_code)]
    pub fn positive_solutions(&self) -> HashSet<(u64, u64)> {
        let mut positive_solutions: HashSet<(u64, u64)> = HashSet::new();

        let k0x: i64 = -(self.x0 / self.b1);
        let k0y: i64 = self.y0 / self.a1;
        let min_k: i64 = min(k0x, k0y) - 1; // -1 just in case
        let max_k: i64 = max(k0x, k0y) + 1; // +1 just in case

        for k in min_k..max_k {
            // Build the next solution
            let xs: i64 = self.x0 + self.b1 * k;
            let ys: i64 = self.y0 - self.a1 * k;

            if xs >= 0 && ys >= 0 {
                positive_solutions.insert((xs as u64, ys as u64));
            }
        }

        positive_solutions
    }
}
