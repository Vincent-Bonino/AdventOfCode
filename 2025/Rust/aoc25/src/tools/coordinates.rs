//! Coordinates utilities.

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::num::TryFromIntError;
use std::ops::{Add, Mul, Sub};

use colored::Colorize;

use super::{
    directions::Direction,
    types::{CoordIntType, DeltaIntType},
};

/// Structure representing coordinates in two dimensions.
#[derive(Copy, Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coordinates {
    /// First coordinate: X or row.
    pub x: CoordIntType,
    /// Second coordinate: Y or column.
    pub y: CoordIntType,
}

impl Coordinates {
    /// Build a coordinate from two [`CoordIntType`].
    ///
    /// See the [`coord macro`](crate::coord) for another way to build coordinates.
    pub fn new<I>(x: I, y: I) -> Self
    where
        I: Into<CoordIntType>,
    {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    // Getters

    pub fn get_row(&self) -> CoordIntType {
        self.x
    }

    pub fn get_column(&self) -> CoordIntType {
        self.y
    }

    // Step

    /// Return the coordinates one step further the provided direction.
    ///
    /// Equivalent to adding the coordinate and [`Direction`] together.
    pub fn step(self, direction: &Direction) -> Self {
        self + direction
    }

    // Neighbours

    /// Return the four usual (cardinal) neighbours.
    pub fn neighbours4(&self) -> Vec<Self> {
        Direction::neighbours4().iter().map(|dir| self.step(dir)).collect()
    }

    /// Return the four diagonal neighbours.
    pub fn neighbours4_diagonal(&self) -> Vec<Self> {
        Direction::neighbours4_diagonal()
            .iter()
            .map(|dir| self.step(dir))
            .collect()
    }

    /// Return all the eight neighbours.
    pub fn neighbours8(&self) -> Vec<Self> {
        Direction::neighbours8().iter().map(|dir| self.step(dir)).collect()
    }

    // Maths-related methods

    #[inline]
    /// Return the difference of coordinates between `self` and `other`.
    /// **Not symmetric !**
    pub fn get_delta(&self, other: &Self) -> (DeltaIntType, DeltaIntType) {
        let dx: DeltaIntType = self.x as DeltaIntType - other.x as DeltaIntType;
        let dy: DeltaIntType = self.y as DeltaIntType - other.y as DeltaIntType;
        (dx, dy)
    }

    #[inline]
    /// Return the Manhattan distance to another coordinates.
    pub fn manhattan_distance_to(&self, other: &Self) -> DeltaIntType {
        let x_dist: DeltaIntType = self.x as DeltaIntType - other.x as DeltaIntType;
        let y_dist: DeltaIntType = self.y as DeltaIntType - other.y as DeltaIntType;
        x_dist.abs() + y_dist.abs()
    }

    // Transformations

    /// Attempt to cast this coordinates' element to [`usize`]s.
    pub fn as_usize_tuple(&self) -> Result<(usize, usize), TryFromIntError> {
        Ok((self.x.try_into()?, self.y.try_into()?))
    }

    /// Attempt to cast this coordinates' element to [`usize`]s.
    /// Panic if unable to.
    pub fn as_usize_tuple_unchecked(&self) -> (usize, usize) {
        self.as_usize_tuple()
            .unwrap_or_else(|_| panic!("Invalid coordinate {self:?}"))
    }
}

#[macro_export]
/// Build [`Coordinates`] casting provided arguments as [`CoordIntType`].
macro_rules! coord {
    ($x:expr, $y:expr) => {
        Coordinates::new($x as CoordIntType, $y as CoordIntType)
    };
}
//pub(crate) use coord;

// --- Trait impls ---

impl Debug for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coord({:?},{:?})", self.x, self.y)
    }
}

/// Display the coordinated with enhanced differentiation on row/column.
impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Coord({},{})",
            self.x.to_string().underline(),
            self.y.to_string().italic()
        )
    }
}

// --- Castings ---

impl<T> From<(T, T)> for Coordinates
where
    T: Into<CoordIntType>,
{
    fn from((x, y): (T, T)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl<T> From<Coordinates> for (T, T)
where
    T: From<CoordIntType>,
{
    fn from(Coordinates { x, y }: Coordinates) -> Self {
        (x.into(), y.into())
    }
}

impl<T> From<[T; 2]> for Coordinates
where
    T: Copy + Into<CoordIntType>,
{
    fn from(val: [T; 2]) -> Self {
        Self {
            x: val[0].into(),
            y: val[1].into(),
        }
    }
}

impl<T> From<Coordinates> for [T; 2]
where
    T: From<CoordIntType>,
{
    fn from(Coordinates { x, y }: Coordinates) -> Self {
        [x.into(), y.into()]
    }
}

// --- Additions ---

/// Implement addition with itself.
impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Implement addition with &[`Direction`].
impl Add<&Direction> for Coordinates {
    type Output = Self;

    #[inline]
    fn add(self, rhs: &Direction) -> Self::Output {
        let deltas: (DeltaIntType, DeltaIntType) = rhs.get_delta();
        Self::Output {
            x: (self.x as DeltaIntType + deltas.0) as CoordIntType,
            y: (self.y as DeltaIntType + deltas.1) as CoordIntType,
        }
    }
}

// --- Substractions ---

/// Implementation of substraction with &[`Direction`].
impl Sub<&Direction> for Coordinates {
    type Output = Self;

    fn sub(self, rhs: &Direction) -> Self::Output {
        let deltas: (DeltaIntType, DeltaIntType) = rhs.get_delta();
        Self::Output {
            x: (self.x as DeltaIntType - deltas.0) as CoordIntType,
            y: (self.y as DeltaIntType - deltas.1) as CoordIntType,
        }
    }
}

// --- Multiplications ---

/// Implementation of multiplication with integers.
impl<T> Mul<T> for Coordinates
where
    T: Copy + Into<CoordIntType>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs.into(),
            y: self.y * rhs.into(),
        }
    }
}

// --- Functions ---

/// Display a hashmap with Coordinates as values.
///
/// Benefits from Coordinates pretty-printing in display, which would have not been used
/// if displaying the collection with debug format.
pub fn display_coords_hashmap<K: Display>(coords: &HashMap<K, Coordinates>) {
    println!(
        "{{ {} }}",
        coords
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join(", ")
    );
}

/// Display a list of Coordinates.
///
/// Benefits from Coordinates pretty-printing in display, which would have not been used
/// if displaying the collection with debug format.
pub fn display_coords_list(coords: &[Coordinates]) {
    println!(
        "[{}]",
        coords
            .iter()
            .map(|coord| coord.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )
}
