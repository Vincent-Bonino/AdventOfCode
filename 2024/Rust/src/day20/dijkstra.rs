use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

use itertools::Itertools;

use crate::day20::model::Tile20;
use crate::toolbox::{Coordinates, Direction, Grid};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pub position: Coordinates,
    pub cost: i64,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn custom_dijkstra(
    grid: &Grid<Tile20>,
    starting_position: &Coordinates,
    exit_position: &Coordinates,
) -> (i64, Vec<Coordinates>, HashMap<Coordinates, i64>) {
    let mut graph: HashMap<Coordinates, Vec<Coordinates>> = HashMap::new();

    // Before anything else, build edges of the graph
    for (coord, tile) in grid.enumerate() {
        if *tile == Tile20::Wall {
            continue;
        }

        let mut neighbours: Vec<Coordinates> = Vec::with_capacity(4);
        for neighb in coord.neighbours4() {
            if let Some(Tile20::Empty) = grid.get_ref(&neighb) {
                neighbours.push(neighb);
            }
        }
        graph.insert(coord, neighbours);
    }

    // --- Initialize values for the algorithm ---

    // Current shortest distance from start
    let mut distances: HashMap<Coordinates, i64> = HashMap::new();
    for key in graph.keys() {
        distances.insert(*key, i64::MAX);
        distances.insert(*key, i64::MAX);
        distances.insert(*key, i64::MAX);
        distances.insert(*key, i64::MAX);
    }

    // Node(s) leading to this node
    let mut previous: HashMap<Coordinates, Coordinates> = HashMap::new();

    // Nodes to progress to
    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    // Add start position, with distance = zero
    distances.insert(*starting_position, 0);
    queue.push(State {
        position: *starting_position,
        cost: 0,
    });

    // --- Run Dijkstra algorithm ---

    while let Some(State { position, cost }) = queue.pop() {
        // Discard worse paths
        if cost > *distances.get(&position).unwrap() {
            continue;
        }

        // Step forward
        for new_position in position.neighbours4() {
            match grid.get_ref(&new_position) {
                None => continue,
                Some(Tile20::Wall) => continue,
                Some(Tile20::Empty) => {}
            }

            if graph.get(&position).unwrap().contains(&new_position) {
                let new_cost: i64 = cost + 1;

                let next: State = State {
                    position: new_position,
                    cost: new_cost,
                };

                if next.cost < *distances.get(&next.position).unwrap() {
                    queue.push(next);

                    // Update with the new (better) path
                    distances.insert(next.position, next.cost);
                    previous.insert(next.position, position);
                }
            }
        }
    }

    let distance: i64 = *distances.get(exit_position).unwrap();

    // Find path going backwards
    let mut path: Vec<Coordinates> = vec![*exit_position];
    let mut current_node: Coordinates = *exit_position;
    while let Some(node) = previous.get(&current_node) {
        path.push(*node);
        current_node = *node;
    }

    (distance, path, distances)
}
