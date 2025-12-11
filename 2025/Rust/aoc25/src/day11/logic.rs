use super::model::{Label2NodeMapping, Node, Node2LabelMapping};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

const P1_START_DEVICE: &str = "you";
const P2_START_DEVICE: &str = "svr";

const P2_NUMBER_REQUIRED: usize = 2;
const P2_REQUIRED: [&str; P2_NUMBER_REQUIRED] = ["dac", "fft"];

const TARGET_DEVICE: &str = "out";

type Depth = usize;
type Weight = usize;

#[expect(unused_assignments)]
pub fn compute_depth(
    graph: &HashMap<Node, Vec<Node>>,
    _node_to_label: &Node2LabelMapping,
    label_to_node: &Label2NodeMapping,
) -> HashMap<Node, Depth> {
    let mut result: HashMap<Node, Depth> = HashMap::with_capacity(graph.len());
    let empty = Vec::new();

    let mut queue: Vec<Node> = Vec::with_capacity(graph.len());
    let mut current_depth: Depth = 0;

    // Compute start
    let start_node: Node = label_to_node[P2_START_DEVICE];

    queue.push(start_node);

    while !queue.is_empty() {
        let mut new_queue: Vec<Node> = Vec::with_capacity(graph.len());

        for device in &queue {
            // Exit condition
            if result.contains_key(device) {
                continue;
            }

            let mut device_depth = result.entry(*device).or_insert(current_depth);
            device_depth = device_depth.max(&mut current_depth);

            for next_device in graph.get(device).unwrap_or(&empty) {
                new_queue.push(*next_device);
            }
        }

        current_depth += 1;
        queue = new_queue;
    }

    result
}

pub fn solve_part_one(
    graph: &HashMap<Node, Vec<Node>>,
    _node_to_label: &Node2LabelMapping,
    label_to_node: &Label2NodeMapping,
) -> String {
    let mut result: usize = 0;
    let empty = Vec::new();

    let mut queue: Vec<Node> = Vec::with_capacity(graph.len());

    // Compute start
    let start_node: Node = label_to_node[P1_START_DEVICE];
    let target_node: Node = label_to_node[TARGET_DEVICE];

    queue.push(start_node);

    while !queue.is_empty() {
        let mut new_queue: Vec<Node> = Vec::with_capacity(graph.len());

        for device in &queue {
            // Exit condition
            if device == &target_node {
                result += 1;
                continue;
            }

            for next_device in graph.get(device).unwrap_or(&empty) {
                let next_device = next_device.to_owned();

                new_queue.push(next_device);
            }
        }

        queue = new_queue;
    }

    result.to_string()
}

pub fn solve_part_two(
    graph: &HashMap<Node, Vec<Node>>,
    node_to_label: &Node2LabelMapping,
    label_to_node: &Label2NodeMapping,
) -> String {
    let depth_mapping: HashMap<Node, Depth> = compute_depth(graph, node_to_label, label_to_node);

    let mut result: usize = 0;
    let empty = Vec::new();

    let mut queue: HashMap<(Node, Vec<bool>), Weight> = HashMap::with_capacity(graph.len());
    let mut current_depth: usize = 0;

    // Compute start
    let start_node: Node = label_to_node[P2_START_DEVICE];
    let target_node: Node = label_to_node[TARGET_DEVICE];

    let required_nodes: Vec<Node> = P2_REQUIRED.iter().map(|&label| label_to_node[label]).collect();

    queue.insert((start_node, vec![false; P2_NUMBER_REQUIRED]), 1);

    while !queue.is_empty() {
        let mut new_queue: HashMap<(Node, Vec<bool>), Weight> = HashMap::with_capacity(graph.len());

        for ((device, mut required), weight) in queue {
            // Exit condition
            if device == target_node {
                if required.iter().all(|&required| required) {
                    result += weight;
                }
                continue;
            }

            // Wait for merge
            if current_depth < depth_mapping[&device] {
                new_queue
                    .entry((device, required))
                    .and_modify(|w| *w += weight)
                    .or_insert(weight);
                continue;
            }

            // Increment
            for req_index in 0..P2_NUMBER_REQUIRED {
                if device == required_nodes[req_index] {
                    required[req_index] = true;
                    break;
                }
            }

            for next_device in graph.get(&device).unwrap_or(&empty) {
                new_queue
                    .entry((*next_device, required.clone()))
                    .and_modify(|w| *w += weight)
                    .or_insert(weight);
            }
        }

        current_depth += 1;
        queue = new_queue;
    }

    result.to_string()
}
