use crate::day16::model::Tile16;
use crate::day16::solve::{STARTING_DIR, STEP_COST, TURN_COST};
use crate::toolbox::{Coordinates, Direction, Grid};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::thread::current;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pub position: Coordinates,
    pub facing: Direction,
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
    maze: &Grid<Tile16>,
    starting_position: &Coordinates,
    exit_position: &Coordinates,
) -> (i64, i64) {
    let mut graph: HashMap<Coordinates, Vec<Coordinates>> = HashMap::new();

    // Before anything else, build edges of the graph
    for (coord, tile) in maze.enumerate() {
        if tile == &Tile16::Wall {
            continue;
        }

        let mut neighbours: Vec<Coordinates> = Vec::with_capacity(4);
        for neighb in coord.neighbours4() {
            if let Some(Tile16::Empty) = maze.get_ref(&neighb) {
                neighbours.push(neighb);
            }
        }
        graph.insert(coord, neighbours);
    }

    // --- Initialize values for the algorithm ---

    // Current shortest distance from start
    let mut distance: HashMap<(Coordinates, Direction), i64> = HashMap::new();
    for key in graph.keys() {
        distance.insert((*key, Direction::N), i64::MAX);
        distance.insert((*key, Direction::S), i64::MAX);
        distance.insert((*key, Direction::W), i64::MAX);
        distance.insert((*key, Direction::E), i64::MAX);
    }

    // Node(s) leading to this node
    #[allow(clippy::type_complexity)]
    let mut previous: HashMap<
        (Coordinates, Direction),
        HashSet<(Coordinates, Direction, i64)>,
    > = HashMap::new();

    // Nodes to progress to
    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    // Add start position, with distance = zero
    distance.insert((*starting_position, STARTING_DIR), 0);
    queue.push(State {
        position: *starting_position,
        facing: STARTING_DIR,
        cost: 0,
    });

    // --- Run Dijkstra algorithm ---

    while let Some(State {
        position,
        facing,
        cost,
    }) = queue.pop()
    {
        // Discard worse paths
        if cost > *distance.get(&(position, facing)).unwrap() {
            continue;
        }

        // Turn left
        {
            let new_direction: Direction = facing.left();
            let new_cost: i64 = cost + TURN_COST;

            let next: State = State {
                position,
                facing: new_direction,
                cost: new_cost,
            };

            if next.cost
                <= *distance
                    .get(&(next.position, next.facing))
                    .expect("Distance lookup failed (left)")
            {
                queue.push(next);

                // Update with the new (better) path
                distance.insert((next.position, next.facing), next.cost);
                previous
                    .entry((next.position, next.facing))
                    .or_default()
                    .insert((position, facing, cost));
            }
        }

        // Turn right
        {
            let new_direction: Direction = facing.right();
            let new_cost: i64 = cost + TURN_COST;

            let next: State = State {
                position,
                facing: new_direction,
                cost: new_cost,
            };

            if next.cost
                <= *distance
                    .get(&(next.position, next.facing))
                    .expect("Distance lookup failed (right)")
            {
                queue.push(next);

                // Update with the new (better) path
                distance.insert((next.position, next.facing), next.cost);
                previous
                    .entry((next.position, next.facing))
                    .or_default()
                    .insert((position, facing, cost));
            }
        }

        // Step forward
        {
            let new_position: Coordinates = position.step(&facing);
            if graph.get(&position).unwrap().contains(&new_position) {
                let new_cost: i64 = cost + STEP_COST;

                let next: State = State {
                    position: new_position,
                    facing,
                    cost: new_cost,
                };

                if next.cost
                    <= *distance
                        .get(&(next.position, next.facing))
                        .expect("Distance lookup failed (step)")
                {
                    queue.push(next);

                    // Update with the new (better) path
                    distance.insert((next.position, next.facing), next.cost);
                    previous
                        .entry((next.position, next.facing))
                        .or_default()
                        .insert((position, facing, cost));
                }
            }
        }
    }

    // --- Part one ---

    let part_one_result: i64 = *distance
        .iter()
        .filter_map(|((key_pos, _key_dir), dist)| {
            if key_pos == exit_position {
                Some(dist)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    // --- Part two ---

    let mut path_coordinates: HashSet<Coordinates> = HashSet::new();

    let mut to_rewind: VecDeque<(Coordinates, Direction, i64)> = VecDeque::new();
    for direction in Direction::neighbours4() {
        let dist = distance
            .get(&(*exit_position, direction))
            .unwrap_or(&i64::MAX);
        if dist == &part_one_result {
            to_rewind.push_back((*exit_position, direction, part_one_result));
        }
    }

    while let Some((cur_pos, cur_dir, cost)) = to_rewind.pop_front() {
        path_coordinates.insert(cur_pos);

        // Add previous
        for (prev_pos, prev_dir, prev_cost) in
            previous.get(&(cur_pos, cur_dir)).unwrap_or(&HashSet::new())
        {
            if *prev_cost == (cost - STEP_COST) || *prev_cost == (cost - TURN_COST) {
                to_rewind.push_back((*prev_pos, *prev_dir, *prev_cost));
            }
        }
    }

    let part_two_result: usize = path_coordinates.len();

    // maze.show_path(&path_coordinates.into_iter().collect_vec(), 'O');

    // --- Return ---
    (part_one_result, part_two_result as i64)
}
