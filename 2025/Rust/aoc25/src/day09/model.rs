use crate::coord;
use crate::prelude::*;
use itertools::Itertools;
use std::ops::{Range, RangeInclusive};

type LineRange = Range<CoordIntType>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Line {
    /// Same X
    HorizontalLine(Coordinates, Coordinates),
    /// Same Y
    VerticalLine(Coordinates, Coordinates),
}

impl Line {
    pub fn new(coord1: Coordinates, coord2: Coordinates) -> Self {
        if coord1.x == coord2.x {
            let min: Coordinates = if coord1.y <= coord2.y { coord1 } else { coord2 };
            let max: Coordinates = if coord1.y >= coord2.y { coord1 } else { coord2 };
            Self::HorizontalLine(min, max)
        } else if coord1.y == coord2.y {
            let min: Coordinates = if coord1.x <= coord2.x { coord1 } else { coord2 };
            let max: Coordinates = if coord1.x >= coord2.x { coord1 } else { coord2 };
            Self::VerticalLine(min, max)
        } else {
            unreachable!()
        }
    }

    pub fn to_coordinates(self) -> Vec<Coordinates> {
        match self {
            Self::HorizontalLine(_, _) => self.range(true).map(|y| coord!(self.value(), y)).collect(),
            Self::VerticalLine(_, _) => self.range(true).map(|x| coord!(x, self.value())).collect(),
        }
    }

    // Utils

    fn range(&self, inclusive: bool) -> LineRange {
        match self {
            Line::HorizontalLine(coord1, coord2) => {
                let (start, end) = if inclusive {
                    (coord1.y, coord2.y + 1)
                } else {
                    (coord1.y + 1, coord2.y)
                };
                start..end
            }
            Line::VerticalLine(coord1, coord2) => {
                let (start, end) = if inclusive {
                    (coord1.x, coord2.x + 1)
                } else {
                    (coord1.x + 1, coord2.x)
                };
                start..end
            }
        }
    }

    fn value(&self) -> CoordIntType {
        match self {
            Line::HorizontalLine(coord, _) => coord.x,
            Line::VerticalLine(coord, _) => coord.y,
        }
    }
}
