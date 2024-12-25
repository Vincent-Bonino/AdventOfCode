use std::io::ErrorKind;

use hashbrown::HashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::IResult;

use crate::day24::model::{CableCircuit, LogicGate};

pub fn parse_input(input: &str) -> CableCircuit {
    let input_parts: Vec<&str> = input.split("\r\n\r\n").collect(); // Windows windows windows ...

    let initial_cable_values: HashMap<String, bool> = parse_cable_values(input_parts[0]);
    let logic_gates: Vec<LogicGate> = parse_logic_gates(input_parts[1]);

    CableCircuit::new(logic_gates, initial_cable_values)
}

fn parse_cable_values(input: &str) -> HashMap<String, bool> {
    let mut result: HashMap<String, bool> = HashMap::new();

    for line in input.lines() {
        let line_parts: Vec<&str> = line.split(": ").collect();
        let cable_name: String = line_parts[0].to_string();
        let cable_value: bool = match line_parts[1] {
            "0" => false,
            "1" => true,
            _ => unreachable!(),
        };

        result.insert(cable_name, cable_value);
    }

    result
}

fn parse_logic_gates(input: &str) -> Vec<LogicGate> {
    let mut result: Vec<LogicGate> = Vec::new();

    for line in input.lines() {
        let (_, gate) = parse_gate(line).unwrap();
        result.push(gate);
    }

    result
}

fn parse_gate(gate_line: &str) -> IResult<&str, LogicGate> {
    map_res(
        tuple((
            alphanumeric1,
            alt((tag(" AND "), tag(" OR "), tag(" XOR "))),
            alphanumeric1,
            tag(" -> "),
            alphanumeric1,
        )),
        |(left, gate, right, _, dest): (&str, &str, &str, &str, &str)| {
            let left: String = left.to_string();
            let right: String = right.to_string();
            let dest: String = dest.to_string();
            Ok::<_, ErrorKind>(match gate {
                " AND " => LogicGate::And(left, right, dest),
                " OR " => LogicGate::Or(left, right, dest),
                " XOR " => LogicGate::Xor(left, right, dest),
                _ => unreachable!(),
            })
        },
    )(gate_line)
}
