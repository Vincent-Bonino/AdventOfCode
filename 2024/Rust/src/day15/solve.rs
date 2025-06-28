use crate::day15::model::Tile15;
use crate::toolbox::{Coordinates, Direction, Grid};

pub fn solve_part_one(
    grid: &Grid<Tile15>,
    robot_position: &Coordinates,
    instructions: &[Direction],
) -> i32 {
    let mut grid: Grid<Tile15> = (*grid).clone();
    let mut current_robot_position: Coordinates = *robot_position;

    // Move the robot following the instructions
    for dir in instructions {
        match grid.can_move_towards_p1(&current_robot_position, dir) {
            None => {}
            Some(nb_boxes) => {
                current_robot_position = current_robot_position.step(dir); // Move the robot
                grid.push_boxes_p1(&current_robot_position, dir, nb_boxes); // THEN push the box(es)
            }
        }
    }

    grid.compute_score()
}

pub fn solve_part_two(
    grid: &Grid<Tile15>,
    robot_position: &Coordinates,
    instructions: &[Direction],
) -> i32 {
    // Expand the input
    let mut expanded_grid: Grid<Tile15> = grid.build_expanded();
    let mut current_robot_position: Coordinates = Coordinates {
        x: robot_position.x,
        y: robot_position.y * 2,
    };

    // Move the robot following the instructions
    for dir in instructions.iter() {
        match expanded_grid.can_move_towards_p2(&current_robot_position, dir) {
            None => {}
            Some((nb_boxes, box_coord)) => {
                current_robot_position = current_robot_position.step(dir); // Move the robot
                expanded_grid.push_boxes_p2(&current_robot_position, dir, nb_boxes, box_coord);
                // THEN push the box(es)
            }
        }
    }

    expanded_grid.compute_score()
}
