use crate::toolbox::{Coordinates, Direction, Grid};
use std::cmp::PartialEq;

#[derive(Debug, Default, Eq, PartialEq)]
pub enum Letter {
    X,
    M,
    A,
    S,
    #[default]
    Irrelevant,
}

impl Grid<Letter> {
    /// Count the number of provided word found, starting at this position
    pub fn count_xmas(&self, coordinates: &Coordinates, expected: &[Letter]) -> i32 {
        let mut counter: i32 = 0;

        // Must start with 'X'
        if self.get_ref(coordinates) != Some(expected.first().expect("Empty expected")) {
            return counter;
        }

        'dir_loop: for direction in Direction::neighbours8() {
            let mut current_position: Coordinates = *coordinates;

            for current_expected in &expected[1..] {
                // Loop increment
                current_position = current_position.step(&direction);

                if self.get_ref(&current_position) != Some(current_expected) {
                    continue 'dir_loop;
                }
            }

            counter += 1
        }

        counter
    }

    /// Determine if at the center of an X-shaped (or +-shaped) 'MAS'
    pub fn is_x_max(&self, coordinates: &Coordinates) -> bool {
        // Must start with 'A'
        if self.get_ref(coordinates) != Some(&Letter::A) {
            return false;
        }

        let mut m_directions: Vec<Direction> = Vec::with_capacity(2);
        let mut s_directions: Vec<Direction> = Vec::with_capacity(2);

        for direction in Direction::neighbours4_diagonal() {
            match self.get_ref(&coordinates.step(&direction)) {
                Some(&Letter::M) => m_directions.push(direction),
                Some(&Letter::S) => s_directions.push(direction),
                _ => {}
            }
        }

        m_directions.len() == 2
            && s_directions.len() == 2
            && &m_directions.first().unwrap().opposite() != m_directions.get(1).unwrap()
    }
}
