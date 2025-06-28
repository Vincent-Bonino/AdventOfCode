use hashbrown::HashMap;
use itertools::{izip, Itertools};
use std::collections::HashSet;
use std::fmt::{write, Debug, Formatter};

#[derive(Clone, Eq, PartialEq)]
pub enum LogicGate {
    And(String, String, String),
    Or(String, String, String),
    Xor(String, String, String),
}

impl LogicGate {
    pub fn as_tuple(&self) -> (&str, &str, &str) {
        match self {
            LogicGate::And(left, right, dest) => (left, right, dest),
            LogicGate::Or(left, right, dest) => (left, right, dest),
            LogicGate::Xor(left, right, dest) => (left, right, dest),
        }
    }

    pub fn inputs(&self) -> (&str, &str) {
        match self {
            LogicGate::And(left, right, _dest) => (left, right),
            LogicGate::Or(left, right, _dest) => (left, right),
            LogicGate::Xor(left, right, _dest) => (left, right),
        }
    }

    pub fn dest(&self) -> &str {
        match self {
            LogicGate::And(_left, _right, dest) => dest,
            LogicGate::Or(_left, _right, dest) => dest,
            LogicGate::Xor(_left, _right, dest) => dest,
        }
    }

    pub fn operation(&self) -> &str {
        match self {
            LogicGate::And(_, _, _) => "AND",
            LogicGate::Or(_, _, _) => "OR",
            LogicGate::Xor(_, _, _) => "XOR",
        }
    }

    pub fn get_other_input(&self, input: &str) -> Option<&str> {
        let (left, right) = self.inputs();
        if left == input {
            Some(right)
        } else if right == input {
            Some(left)
        } else {
            None
        }
    }
}

impl Debug for LogicGate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicGate::And(left, right, dest) => write!(f, "{left} AND {right} -> {dest}"),
            LogicGate::Or(left, right, dest) => write!(f, "{left} OR {right} -> {dest}"),
            LogicGate::Xor(left, right, dest) => write!(f, "{left} XOR {right} -> {dest}"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CableCircuit {
    pub cable_values: HashMap<String, bool>,
    pub logic_gates: Vec<LogicGate>,
}

impl CableCircuit {
    pub fn new(logic_gates: Vec<LogicGate>, initial_cable_values: HashMap<String, bool>) -> Self {
        Self {
            logic_gates,
            cable_values: initial_cable_values,
        }
    }

    pub fn simulate(&mut self) {
        loop {
            let mut continue_loop: bool = false;

            for gate in &self.logic_gates {
                let (left, right, dest): (&str, &str, &str) = gate.as_tuple();

                if !self.cable_values.contains_key(dest)
                    && self.cable_values.contains_key(left)
                    && self.cable_values.contains_key(right)
                {
                    let left_val: &bool = self.cable_values.get(left).unwrap();
                    let right_val: &bool = self.cable_values.get(right).unwrap();

                    self.cable_values.insert(
                        String::from(dest),
                        match gate {
                            LogicGate::And(_, _, _) => left_val & right_val,
                            LogicGate::Or(_, _, _) => left_val | right_val,
                            LogicGate::Xor(_, _, _) => left_val ^ right_val,
                        },
                    );
                    continue_loop = true;
                }
            }

            if !continue_loop {
                break;
            }
        }
    }

    // Part two

    pub fn inspect_shape(&self) {
        let x_nodes: Vec<String> = self.generate_cables_starting_with("x");
        let y_nodes: Vec<String> = self.generate_cables_starting_with("y");
        let z_nodes: Vec<String> = self.generate_cables_starting_with("z");

        let mut carry_cable: Option<&str> = None;
        let mut errored_cables: HashSet<String> = HashSet::with_capacity(8);

        for (_index, x_node, y_node, z_node) in izip!(0.., x_nodes, y_nodes, z_nodes).skip(2) {
            // println!("\nLooking for index={index}, carry = {carry_cable:?}");

            let x_and_y_gate = self
                .get_gate_from_inputs(Some(&x_node), Some(&y_node), "AND")
                .expect("Base AND gate missing");
            let x_and_y_node = x_and_y_gate.dest();

            let x_xor_y_gate = self
                .get_gate_from_inputs(Some(&x_node), Some(&y_node), "XOR")
                .expect("BASE XOR gate missing");
            let x_xor_y_node = x_xor_y_gate.dest();

            //  1. Ensure X&Y goes to a OR

            let _x_and_y_next_or = match self.get_gate_from_input(Some(x_and_y_node), "OR") {
                None => {
                    // println!("[E] (X&Y) ({x_and_y_node:?}) is not linked to an 'OR'");
                    errored_cables.insert(x_and_y_node.to_string());
                    None
                }
                Some(x) => Some(x),
            };

            //  2. Ensure X^Y goes to a XOR

            let x_xor_y_next_xor = match self.get_gate_from_input(Some(x_xor_y_node), "XOR") {
                None => {
                    // println!("[E] (X^Y) ({x_xor_y_node:?}) is not linked to a 'XOR'");
                    errored_cables.insert(x_xor_y_node.to_string());
                    None
                }
                Some(x) => Some(x),
            };
            let x_xor_y_next_xor_other_input = match x_xor_y_next_xor {
                None => None,
                Some(gate) => gate.get_other_input(x_xor_y_node),
            };

            //  2.5 Ensure the result of the XOR is Z

            if let Some(gate) = x_xor_y_next_xor {
                if gate.dest() != z_node {
                    // println!("[E] Expected XOR gate to be linked to Z");
                    errored_cables.insert(z_node.clone());
                    errored_cables.insert(gate.dest().to_string());
                }
            }

            //  3. Ensure X^Y goes to an AND

            let x_xor_y_next_and = match self.get_gate_from_input(Some(x_xor_y_node), "AND") {
                None => {
                    // println!("[E] (X^Y) ({x_xor_y_node:?}) is not linked to a 'AND'");
                    errored_cables.insert(x_xor_y_node.to_string());
                    None
                }
                Some(x) => Some(x),
            };
            let x_xor_y_next_and_other_input = match x_xor_y_next_and {
                None => None,
                Some(gate) => gate.get_other_input(x_xor_y_node),
            };

            //  4. Ensure both previous gates are linked to the same node (the input carry)

            if let (Some(other_xor_input), Some(other_and_input)) =
                (x_xor_y_next_xor_other_input, x_xor_y_next_and_other_input)
            {
                if let Some(carry) = carry_cable {
                    if other_xor_input != carry {
                        // println!("[E] Carry not linked to XOR");
                        errored_cables.insert(carry.to_string());
                        errored_cables.insert(other_xor_input.to_string());
                    }
                    if other_and_input != carry {
                        // println!("[E] Carry not linked to AND");
                        errored_cables.insert(carry.to_string());
                        errored_cables.insert(other_and_input.to_string());
                    }
                }
            }

            // Attempt to build the carry for the next step

            let gate_to_z_from_z = self.get_gate_from_dest(&z_node);

            let input_carry_node = match carry_cable {
                Some(_) => carry_cable,
                None => gate_to_z_from_z.get_other_input(x_xor_y_node),
            };

            let intermediate_carry_gate =
                self.get_gate_from_inputs(input_carry_node, Some(x_xor_y_node), "AND");
            let intermediate_carry_node = intermediate_carry_gate.map(|gate| gate.dest());
            let output_carry_gate =
                self.get_gate_from_inputs(intermediate_carry_node, Some(x_and_y_node), "OR");

            carry_cable = if let Some(gate) = output_carry_gate {
                Some(gate.dest())
            } else {
                None
            };
        }

        println!("Swapped wires ({}):", errored_cables.len());
        println!("{}", errored_cables.iter().sorted().join(","));
    }

    fn get_gate_from_input<'a>(
        &'a self,
        input1: Option<&str>,
        operation: &str,
    ) -> Option<&'a LogicGate> {
        let input1 = input1?;

        for gate in self.logic_gates.iter() {
            // Filter on operation
            if gate.operation() != operation {
                continue;
            }

            // Filter on inputs
            let (left, right) = gate.inputs();
            if left == input1 || right == input1 {
                return Some(gate);
            }
        }
        None
    }

    fn get_gate_from_inputs<'a>(
        &'a self,
        input1: Option<&str>,
        input2: Option<&str>,
        operation: &str,
    ) -> Option<&'a LogicGate> {
        let input1 = input1?;
        let input2 = input2?;

        let target_input = [input1, input2];

        for gate in self.logic_gates.iter() {
            // Filter on operation
            if gate.operation() != operation {
                continue;
            }

            // Filter on inputs
            let (left, right) = gate.inputs();
            if target_input.contains(&left) && target_input.contains(&right) {
                return Some(gate);
            }
        }
        None
    }

    fn get_gate_from_dest(&self, dest: &str) -> &LogicGate {
        for gate in self.logic_gates.iter() {
            if gate.dest() == dest {
                return gate;
            }
        }
        panic!("Not found")
    }

    // Utils

    fn generate_cables_starting_with(&self, value: &str) -> Vec<String> {
        let cable_count: usize = self
            .cable_values
            .keys()
            .filter(|key| key.starts_with(value))
            .count();
        let count_log10: u32 = cable_count.ilog10();

        (0..cable_count)
            .map(|i| match count_log10 {
                0 => format!("{value}{i}"),
                1 => format!("{value}{i:0>2}"),
                2 => format!("{value}{i:0>3}"),
                _ => unimplemented!("Too many values"),
            })
            .collect()
    }

    pub fn get_output(&self) -> i64 {
        let mut result: i64 = 0;

        for (i, z_cable) in self.generate_cables_starting_with("z").iter().enumerate() {
            if *self.cable_values.get(z_cable).unwrap() {
                result += 1 << i;
            }
        }

        result
    }
}
