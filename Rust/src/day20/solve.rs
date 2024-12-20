use crate::day20::compute::{compute_reachable, count_shortcuts_above};
use crate::day20::dijkstra::custom_dijkstra;
use crate::day20::model::Tile20;
use crate::toolbox::{Coordinates, Grid};
use std::collections::HashMap;

const PART_ONE_CHEAT_LENGTH: i64 = 2;
const PART_ONE_MINIMUM_TIME_SAVE: i64 = 100;

const PART_TWO_CHEAT_LENGTH: i64 = 20;
const PART_TWO_MINIMUM_TIME_SAVE: i64 = 100;

pub fn solve_part_one(track: &Grid<Tile20>, start: &Coordinates, end: &Coordinates) -> u64 {
    let (_len, path, distances): (i64, Vec<Coordinates>, HashMap<Coordinates, i64>) =
        custom_dijkstra(track, start, end);
    let reachable = compute_reachable(&path, PART_ONE_CHEAT_LENGTH);
    count_shortcuts_above(&reachable, &distances, PART_ONE_MINIMUM_TIME_SAVE)
}

pub fn solve_part_two(track: &Grid<Tile20>, start: &Coordinates, end: &Coordinates) -> u64 {
    let (_len, path, distances): (i64, Vec<Coordinates>, HashMap<Coordinates, i64>) =
        custom_dijkstra(track, start, end);
    let reachable = compute_reachable(&path, PART_TWO_CHEAT_LENGTH);
    count_shortcuts_above(&reachable, &distances, PART_TWO_MINIMUM_TIME_SAVE)
}
