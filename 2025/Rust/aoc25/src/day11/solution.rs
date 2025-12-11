use hashbrown::HashMap;

use aoc25_macros::Aoc25Day;

use super::logic::{solve_part_one, solve_part_two};
use super::model::{Label, Label2NodeMapping, Node, Node2LabelMapping};
use super::parsing::parse_input;
use crate::prelude::*;

#[derive(Default, Aoc25Day)]
/// Solution for day 11.
pub struct Day11 {
    graph: HashMap<Node, Vec<Node>>,
    node_to_label: Node2LabelMapping,
    label_to_node: Label2NodeMapping,
}

impl Aoc25Solution for Day11 {
    fn parse_input(&mut self, input: String) {
        let (graph, node_to_label, label_to_node) = parse_input(&input);
        self.graph = graph;
        self.node_to_label = node_to_label;
        self.label_to_node = label_to_node;
    }

    fn solve_part_one(&mut self) -> Option<String> {
        Some(solve_part_one(&self.graph, &self.node_to_label, &self.label_to_node))
    }

    fn solve_part_two(&mut self) -> Option<String> {
        Some(solve_part_two(&self.graph, &self.node_to_label, &self.label_to_node))
    }
}
