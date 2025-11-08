//! Direction utilities.

use std::fmt::{Display, Formatter};

use crate::tools::types::DeltaIntType;

/// Enumeration representing the 8 directions of a square grid.
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    /// Build the direction from the offset it represents on a 2D grid.
    pub fn from_delta(delta: (DeltaIntType, DeltaIntType)) -> Self {
        match delta {
            (-1, 0) => Self::N,
            (-1, 1) => Self::NE,
            (0, 1) => Self::E,
            (1, 1) => Self::SE,
            (1, 0) => Self::S,
            (1, -1) => Self::SW,
            (0, -1) => Self::W,
            (-1, -1) => Self::NW,
            _ => panic!("Invalid delta for direction: {delta:?}"),
        }
    }

    /// Build the offset it represents on a 2D grid.
    pub fn get_delta(&self) -> (DeltaIntType, DeltaIntType) {
        match self {
            Self::N => (-1, 0),
            Self::NE => (-1, 1),
            Self::E => (0, 1),
            Self::SE => (1, 1),
            Self::S => (1, 0),
            Self::SW => (1, -1),
            Self::W => (0, -1),
            Self::NW => (-1, -1),
        }
    }

    /// Get the direction 90° to the left.
    ///
    /// **Only implemented for cardinal directions.**
    pub fn left(&self) -> Self {
        match self {
            Self::N => Self::W,
            Self::W => Self::S,
            Self::S => Self::E,
            Self::E => Self::N,
            _ => unimplemented!("No left() for diagonal directions"),
        }
    }

    /// Get the opposite direction.
    pub fn opposite(&self) -> Self {
        match self {
            Self::N => Self::S,
            Self::NE => Self::SW,
            Self::E => Self::W,
            Self::SE => Self::NW,
            Self::S => Self::N,
            Self::SW => Self::NE,
            Self::W => Self::E,
            Self::NW => Self::SE,
        }
    }

    /// Get the direction 90° to the right.
    ///
    /// **Only implemented for cardinal directions.**
    pub fn right(&self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
            _ => unimplemented!("No right() for diagonal directions"),
        }
    }

    // Neighbours

    /// Return the four usual neighbour-related directions (N; E; S; W).
    pub fn neighbours4() -> Vec<Self> {
        vec![Self::N, Self::E, Self::S, Self::W]
    }

    /// Return the four diagonal neighbour-related directions (NE; SE; NW; SW).
    pub fn neighbours4_diagonal() -> Vec<Self> {
        vec![Self::NE, Self::SE, Self::SW, Self::NW]
    }

    /// Return all the possible directions.
    pub fn neighbours8() -> Vec<Self> {
        vec![
            Self::N,
            Self::NE,
            Self::E,
            Self::SE,
            Self::S,
            Self::SW,
            Self::W,
            Self::NW,
        ]
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::N => write!(f, "^"),
            Self::NE => write!(f, "↗"),
            Self::E => write!(f, ">"),
            Self::SE => write!(f, "↘"),
            Self::S => write!(f, "v"),
            Self::SW => write!(f, "↙"),
            Self::W => write!(f, "<"),
            Self::NW => write!(f, "↖"),
        }
    }
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "N" | "^" => Self::N,
            "NE" => Self::NE,
            "E" | ">" => Self::E,
            "SE" => Self::SE,
            "S" | "v" => Self::S,
            "SW" => Self::SW,
            "W" | "<" => Self::W,
            "NW" => Self::NW,
            _ => unreachable!("Invalid direction '{}'", value),
        }
    }
}
