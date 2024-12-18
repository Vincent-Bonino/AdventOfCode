use crate::toolbox::Direction;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Coordinates {
    /// Return the difference of coordinates between `self` and `other`.
    /// Not symmetric !
    pub fn get_delta(&self, other: &Self) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }

    pub fn neighbours4(&self) -> Vec<Coordinates> {
        Direction::neighbours4()
            .iter()
            .map(|dir| self.step(dir))
            .collect()
    }

    pub fn neighbours4_diagonal(&self) -> Vec<Coordinates> {
        Direction::neighbours4_diagonal()
            .iter()
            .map(|dir| self.step(dir))
            .collect()
    }

    pub fn neighbours8(&self) -> Vec<Coordinates> {
        Direction::neighbours8()
            .iter()
            .map(|dir| self.step(dir))
            .collect()
    }

    pub fn with_delta(&self, delta: (i32, i32), add: bool) -> Self {
        match add {
            true => Coordinates {
                x: self.x + delta.0,
                y: self.y + delta.1,
            },
            false => Coordinates {
                x: self.x - delta.0,
                y: self.y - delta.1,
            },
        }
    }

    pub fn step(&self, direction: &Direction) -> Self {
        let (delta_x, delta_y): (i32, i32) = direction.get_delta();
        Coordinates {
            x: self.x + delta_x,
            y: self.y + delta_y,
        }
    }
}

impl Debug for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coord({},{})", self.x, self.y)
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i32> for Coordinates {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
