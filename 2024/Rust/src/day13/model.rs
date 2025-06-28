use std::cmp::{max, min};
use std::collections::HashSet;

use faer::mat;
use faer::prelude::*;

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

    /// Determine the token price to pay to obtain the prize.
    ///
    /// Return 0 if the prize is unreachable.
    ///
    /// Used crate: https://github.com/sarah-quinones/faer-rs
    pub fn get_token_price(&self) -> u64 {
        let a = mat![
            [self.button_a.0 as f64, self.button_b.0 as f64],
            [self.button_a.1 as f64, self.button_b.1 as f64]
        ];
        let b = mat![[self.prize.0 as f64], [self.prize.1 as f64]];

        // Compute the LU decomposition with partial pivoting,
        let a_plu = a.partial_piv_lu();
        let sol = a_plu.solve(&b);

        // Verify the solution is valid
        let nb_press_a: u64 = sol.get(0, 0).round() as u64;
        let nb_press_b: u64 = sol.get(1, 0).round() as u64;

        let pxs: u64 = self.button_a.0 * nb_press_a + self.button_b.0 * nb_press_b;
        let pys: u64 = self.button_a.1 * nb_press_a + self.button_b.1 * nb_press_b;

        if (pxs, pys) == self.prize {
            ClawMachine::price(&nb_press_a, &nb_press_b)
        } else {
            0
        }
    }
}
