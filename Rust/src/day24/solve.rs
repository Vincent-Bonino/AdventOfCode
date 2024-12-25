use hashbrown::{HashMap, HashSet};
use petgraph::data::Build;
use petgraph::dot::{Config, Dot};
use petgraph::Graph;

use crate::day24::model::CableCircuit;

pub fn solve_part_one(cable_circuit: &CableCircuit) -> i64 {
    let mut cable_circuit: CableCircuit = (*cable_circuit).clone();
    cable_circuit.simulate();
    cable_circuit.get_output()
}

/// Solve part two, see code in [`CableCircuit::inspect_shape`]
///
/// Binary adder with logic gate is a rather structured circuit,
/// that will repeat itself (see [`Binary adder image`]).
///
/// Basically, if adding X and Y to Z, with C the carry
///  - Zi   = Xi ^ Yi ^ Ci
///  - Ci+1 = Xi&Yi | Xi&Ci | Yi&Ci
///
///  As we are told to find `8` cables being swapped,
/// it means that the binary adder is already almost working.
///
/// This solution works with a few hypothesis:
///  1. It does not integrate validation for first two bits and overflow carry.
///     However, it could easily be computed after/before and those are simpler cases.
///  2. It works on the adder as independent (except the carry) blocks,
///     meaning that it will only attempt to detect the pattern (see image) in each block.
///
/// Limitations:
///  - It is (probably?) not able to find errors mixing different levels
///  - It could (probably?) be tricked with two following blocks having errors,
///    if the errors are specially conceived to trick it, with the recovery of the carry in case of error.
///  - Some error detections are not implemented, but this is only is list of conditions to check,
///    so it could be improved to detect errors found on other inputs (e.g. if found only 3 pairs of cable).
///
/// [`Binary adder image`]: https://media.geeksforgeeks.org/wp-content/uploads/20240404130934/Binary-Adder-with-Logic-Gates.png
pub fn solve_part_two(cable_circuit: &CableCircuit) -> i64 {
    // Run the simulation to make every cable appear
    let mut cable_circuit: CableCircuit = (*cable_circuit).clone();
    cable_circuit.simulate();

    // Plot the graph
    // print_graph_dot(&cable_circuit);

    // Solve
    cable_circuit.inspect_shape();

    -2
}

#[allow(dead_code)]
fn print_graph_dot(cable_circuit: &CableCircuit) {
    let mut graph: Graph<&str, ()> = Graph::<_, _>::new();

    let nodes: HashMap<_, _> = HashMap::from_iter(
        cable_circuit
            .cable_values
            .keys()
            .map(|key| (key.as_str(), graph.add_node(key))),
    );

    for gate in cable_circuit.logic_gates.iter() {
        let (left, right, dest) = gate.as_tuple();

        let left_node = nodes[left];
        let right_node = nodes[right];
        let dest_node = nodes[dest];

        let operation_node = graph.add_node(gate.operation());

        graph.add_edge(left_node, operation_node, ());
        graph.add_edge(right_node, operation_node, ());
        graph.add_edge(operation_node, dest_node, ());
    }
    println!("{:?}", Dot::with_config(&graph, &[]));
}
