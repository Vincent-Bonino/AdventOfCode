use std::cmp::PartialEq;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Formatter};

use itertools::Itertools;

use crate::day15::parse::GRID_SIZE;
use crate::toolbox::{Coordinates, Direction, Grid};

const X_WEIGHT: i32 = 100;
const Y_WEIGHT: i32 = 1;

#[derive(Clone, Default, PartialEq)]
pub enum Tile15 {
    #[default]
    Empty,
    Wall,

    // Part one
    Box,

    // Part two
    BoxLeft,
    BoxRight,
}

impl Debug for Tile15 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Box => write!(f, "0"),
            Self::Empty => write!(f, "."),
            Self::Wall => write!(f, "#"),
            Self::BoxLeft => write!(f, "["),
            Self::BoxRight => write!(f, "]"),
        }
    }
}

impl Grid<Tile15> {
    // ----- Part 1 methods -----

    /// Determine if the robot can move in the given direction.
    ///
    /// Returns None if it can't, Some(x) otherwise with x being the number of boxes to push.
    pub fn can_move_towards_p1(
        &self,
        robot_position: &Coordinates,
        direction: &Direction,
    ) -> Option<usize> {
        let mut next_coord: Coordinates = robot_position.step(direction);
        let mut nb_boxes: usize = 0;

        loop {
            match self.get_ref(&next_coord) {
                None => return None,                          // Would move OOB I guess ?
                Some(Tile15::Empty) => return Some(nb_boxes), // Free space ahead, can move (and potentially move boxes)
                Some(Tile15::Wall) => return None,
                Some(Tile15::Box) => nb_boxes += 1, // Continue, to see if boxes have a free space to be moved
                _ => unreachable!("Invalid P1 tile"),
            };
            next_coord = next_coord.step(direction);
        }
    }

    pub fn push_boxes_p1(
        &mut self,
        new_robot_pos: &Coordinates,
        direction: &Direction,
        nb_boxes: usize,
    ) {
        if nb_boxes > 0 {
            // The new robot position becomes empty
            self.data[new_robot_pos.x as usize][new_robot_pos.y as usize] = Tile15::Empty;

            // And a box "appears" at the end of the box chain
            let delta: (i32, i32) = direction.get_delta();
            let new_box_x: i32 = new_robot_pos.x + (nb_boxes as i32) * delta.0;
            let new_box_y: i32 = new_robot_pos.y + (nb_boxes as i32) * delta.1;

            self.data[new_box_x as usize][new_box_y as usize] = Tile15::Box;
        }
    }

    // ----- Part 2 methods -----

    /// Create an expanded copy of the current grid.
    pub fn build_expanded(&self) -> Grid<Tile15> {
        let mut expanded_data: Vec<Vec<Tile15>> = Vec::with_capacity(GRID_SIZE * 2);

        for i in 0..self.depth() {
            let mut expanded_line: Vec<Tile15> = Vec::with_capacity(GRID_SIZE * 2);

            for j in 0..self.width() {
                let new_tiles: Vec<Tile15> = match self.get_ref(&Coordinates {
                    x: i as i32,
                    y: j as i32,
                }) {
                    Some(Tile15::Empty) => vec![Tile15::Empty; 2],
                    Some(Tile15::Wall) => vec![Tile15::Wall; 2],
                    Some(Tile15::Box) => vec![Tile15::BoxLeft, Tile15::BoxRight],
                    _ => unreachable!("Error while expanding"),
                };
                expanded_line.extend(new_tiles);
            }

            expanded_data.push(expanded_line);
        }

        Grid::new(expanded_data)
    }

    // To keep the easy push algorithm from part one:
    //  - I applied it (slightly modified) to horizontal pushes
    //  - I implemented something else for vertical pushes
    // Was it a good idea ? One might never know...

    pub fn can_move_towards_p2(
        &self,
        robot_position: &Coordinates,
        direction: &Direction,
    ) -> Option<(usize, HashSet<Coordinates>)> {
        match direction {
            Direction::N | Direction::S => self.can_move_towards_ver_p2(robot_position, direction),
            Direction::W | Direction::E => self.can_move_towards_hor_p2(robot_position, direction),
            _ => unreachable!(),
        }
    }

    pub fn push_boxes_p2(
        &mut self,
        new_robot_pos: &Coordinates,
        direction: &Direction,
        nb_boxes: usize,
        box_coord: HashSet<Coordinates>,
    ) {
        match direction {
            Direction::N | Direction::S => {
                self.push_boxes_ver_p2(new_robot_pos, direction, nb_boxes, box_coord)
            }
            Direction::W | Direction::E => {
                self.push_boxes_hor_p2(new_robot_pos, direction, nb_boxes, box_coord)
            }
            _ => unreachable!(),
        }
    }

    // Easier case of horizontal movement

    /// Determine if the robot can move in the given (horizontal) direction.
    ///
    /// Returns None if it can't, Some((x, _)) otherwise with x being the number of boxes to push.
    fn can_move_towards_hor_p2(
        &self,
        robot_position: &Coordinates,
        direction: &Direction,
    ) -> Option<(usize, HashSet<Coordinates>)> {
        let mut next_coord: Coordinates = robot_position.step(direction);
        let mut nb_boxes: usize = 0;

        loop {
            match self.get_ref(&next_coord) {
                None => return None, // Would move OOB I guess ?
                Some(Tile15::Empty) => return Some((nb_boxes, HashSet::new())), // Free space ahead, can move (and potentially move boxes)
                Some(Tile15::Wall) => return None,
                Some(Tile15::BoxLeft) => nb_boxes += 1, // Continue, to see if boxes have a free space to be moved
                Some(Tile15::BoxRight) => {} // Continue, to see if boxes have a free space to be moved
                _ => unreachable!("Invalid P2 tile"),
            };
            next_coord = next_coord.step(direction);
        }
    }

    fn push_boxes_hor_p2(
        &mut self,
        new_robot_pos: &Coordinates,
        direction: &Direction,
        nb_boxes: usize,
        _box_coord: HashSet<Coordinates>,
    ) {
        if nb_boxes > 0 {
            // The new robot position becomes empty
            self.data[new_robot_pos.x as usize][new_robot_pos.y as usize] = Tile15::Empty;

            let delta: (i32, i32) = direction.get_delta();

            // Boxes are pushed, so their left/right sides are changed
            for i in 0..(2 * nb_boxes) {
                let new_box_x: i32 = new_robot_pos.x + (i as i32) * delta.0;
                let new_box_y: i32 = new_robot_pos.y + (i as i32) * delta.1;

                let cur_tile: &Tile15 = &self.data[new_box_x as usize][new_box_y as usize];
                self.data[new_box_x as usize][new_box_y as usize] = match cur_tile {
                    Tile15::BoxLeft => Tile15::BoxRight, // Pushing to the right
                    Tile15::BoxRight => Tile15::BoxLeft, // Pushing to the left
                    Tile15::Empty => Tile15::Empty,
                    _ => unreachable!(
                        "Invalid tile '{cur_tile:?}' ({i}/{}) at {}, {}",
                        2 * nb_boxes,
                        new_box_x,
                        new_box_y
                    ),
                };
            }

            // And a box "appears" at the end of the box chain
            let new_box_x: i32 = new_robot_pos.x + (nb_boxes as i32) * 2 * delta.0;
            let new_box_y: i32 = new_robot_pos.y + (nb_boxes as i32) * 2 * delta.1;

            self.data[new_box_x as usize][new_box_y as usize] = match direction {
                Direction::E => Tile15::BoxRight, // Pushing to the right
                Direction::W => Tile15::BoxLeft,  // Pushing to the left
                _ => unreachable!("Invalid horizontal direction {direction:?}"),
            };
        }
    }

    // Harder case of vertical movement

    /// Determine if the robot can move in the given (vertical) direction.
    ///
    /// Returns None if it can't, Some((_, coord)) otherwise with coord being the boxes parts to move.
    fn can_move_towards_ver_p2(
        &self,
        robot_position: &Coordinates,
        direction: &Direction,
    ) -> Option<(usize, HashSet<Coordinates>)> {
        let mut coord_with_boxes_to_move: HashSet<Coordinates> = HashSet::new();
        let mut coord_to_investigate: VecDeque<Coordinates>;

        let next_robot_position: Coordinates = robot_position.step(direction);
        coord_to_investigate = match self.get_ref(&next_robot_position) {
            Some(Tile15::Empty) => return Some((0, coord_with_boxes_to_move)), // Early return, can move without moving
            Some(Tile15::Wall) => return None, // Early return, not move
            Some(Tile15::BoxLeft) => {
                VecDeque::from([next_robot_position, next_robot_position.step(&Direction::E)])
            }
            Some(Tile15::BoxRight) => {
                VecDeque::from([next_robot_position, next_robot_position.step(&Direction::W)])
            }
            _ => unreachable!(),
        };

        let left_dir: Direction = match direction {
            Direction::N => Direction::NW,
            Direction::S => Direction::SW,
            _ => unreachable!(),
        };
        let right_dir: Direction = match direction {
            Direction::N => Direction::NE,
            Direction::S => Direction::SE,
            _ => unreachable!(),
        };

        while !coord_to_investigate.is_empty() {
            let current_coord: Coordinates = coord_to_investigate.pop_front().unwrap();
            if coord_with_boxes_to_move.contains(&current_coord) {
                continue;
            }

            match self.get_ref(&current_coord) {
                None => return None, // Would move OOB I guess ?
                Some(Tile15::Wall) => {
                    // A single blocked box prevents moving everything
                    return None;
                }
                Some(Tile15::Empty) => {
                    // This tile is free for a box part, what about the rest of it ?
                }
                // If founding a box (part), look behind and next to it
                Some(Tile15::BoxLeft) => {
                    coord_with_boxes_to_move.insert(current_coord);
                    coord_with_boxes_to_move.insert(current_coord.step(&Direction::E));
                    coord_to_investigate.push_back(current_coord.step(direction));
                    coord_to_investigate.push_back(current_coord.step(&right_dir));
                }
                Some(Tile15::BoxRight) => {
                    coord_with_boxes_to_move.insert(current_coord);
                    coord_with_boxes_to_move.insert(current_coord.step(&Direction::W));
                    coord_to_investigate.push_back(current_coord.step(direction));
                    coord_to_investigate.push_back(current_coord.step(&left_dir));
                }
                _ => unreachable!("Invalid P2 tile"),
            };
        }

        // Moving is possible
        Some((0, coord_with_boxes_to_move))
    }

    fn push_boxes_ver_p2(
        &mut self,
        _new_robot_pos: &Coordinates,
        direction: &Direction,
        _nb_boxes: usize,
        box_coord: HashSet<Coordinates>,
    ) {
        if !box_coord.is_empty() {
            let sort_function = match direction {
                Direction::N => |coord: &Coordinates| coord.x,
                Direction::S => |coord: &Coordinates| -coord.x,
                _ => unreachable!(),
            };
            let sorted_box_coord: Vec<Coordinates> =
                box_coord.into_iter().sorted_by_key(sort_function).collect();

            // Move boxes
            let data_copy = self.data.clone();
            for box_coord in sorted_box_coord.iter() {
                let target_coord: Coordinates = box_coord.step(direction);

                self.replace(box_coord, Tile15::Empty);
                self.replace(
                    &target_coord,
                    data_copy[box_coord.x as usize][box_coord.y as usize].clone(),
                );
            }
        }
    }

    // Generic methods

    pub fn compute_score(&self) -> i32 {
        let mut result: i32 = 0;

        for (coord, value) in self.enumerate() {
            // P1
            if let Tile15::Box = value {
                result += coord.x * X_WEIGHT + coord.y * Y_WEIGHT;
            }
            // P2
            else if let Tile15::BoxLeft = value {
                result += coord.x * X_WEIGHT + coord.y * Y_WEIGHT;
            }
        }

        result
    }

    // Debug

    /// Guess what, I managed to make boxes disappear.
    pub fn count_boxes(&self) -> usize {
        self.data
            .iter()
            .flatten()
            .filter(|tile| Tile15::BoxLeft == **tile)
            .count()
    }

    /// I might also have broken one or two boxes...
    pub fn integrity_check(&self) -> bool {
        for i in 0..self.depth() {
            for j in 0..self.width() {
                let current_coord = Coordinates {
                    x: i as i32,
                    y: j as i32,
                };
                let current_value = self.get_ref(&current_coord).unwrap();

                if current_value == &Tile15::BoxLeft
                    && self.get_ref(&current_coord.step(&Direction::E)).unwrap()
                        != &Tile15::BoxRight
                {
                    println!("Invalid at {current_coord:?}");
                    return false;
                }
                if current_value == &Tile15::BoxRight
                    && self.get_ref(&current_coord.step(&Direction::W)).unwrap() != &Tile15::BoxLeft
                {
                    println!("Invalid at {current_coord:?}");
                    return false;
                }
            }
        }

        true
    }
}
