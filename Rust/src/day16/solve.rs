/// The "solving" is done in dijkstra.rs
use crate::day16::dijkstra::custom_dijkstra;
use crate::day16::model::Tile16;
use crate::toolbox::{Coordinates, Direction, Grid};
use std::collections::{BinaryHeap, HashMap};

pub const STEP_COST: i64 = 1;
pub const TURN_COST: i64 = 1_000;

pub const STARTING_DIR: Direction = Direction::E;
