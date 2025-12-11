use hashbrown::HashMap;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::combinator::map_res;
use nom::error::ErrorKind;
use nom::multi::{many, many1, separated_list1};
use nom::sequence::terminated;
use nom::{IResult, Parser};

use super::model::{Label2NodeMapping, Node, Node2LabelMapping};

pub fn parse_input(input: &str) -> (HashMap<Node, Vec<Node>>, Node2LabelMapping, Label2NodeMapping) {
    let device_mapping: Vec<(&str, Vec<&str>)> = many1(parse_line).parse(input).unwrap().1;

    // Mappings between label and node
    let mut node_to_label: Node2LabelMapping = HashMap::new();
    let mut label_to_node: Label2NodeMapping = HashMap::new();
    // Graph
    let mut result: HashMap<Node, Vec<Node>> = HashMap::new();

    let mut next_node: Node = 0;
    for (device, next_devices) in device_mapping.into_iter() {
        let device_id: Node = *label_to_node.entry(device.to_owned()).or_insert_with(|| {
            let value = next_node;
            next_node += 1;
            value
        });

        let next_devices_ids: Vec<Node> = next_devices
            .iter()
            .map(|&nd| {
                *label_to_node.entry(nd.to_owned()).or_insert_with(|| {
                    let value = next_node;
                    next_node += 1;
                    value
                })
            })
            .collect();

        // Map
        node_to_label.entry(device_id).or_insert(device.to_owned());
        for (i, next_device_id) in next_devices_ids.iter().enumerate() {
            node_to_label
                .entry(*next_device_id)
                .or_insert(next_devices[i].to_owned());
        }

        // Compute
        result.insert(device_id, next_devices_ids);
    }

    (result, node_to_label, label_to_node)
}

fn parse_line(line: &str) -> IResult<&str, (&str, Vec<&str>)> {
    map_res(
        terminated(
            (
                parse_device_label,
                tag(": "),
                separated_list1(tag(" "), parse_device_label),
            ),
            line_ending,
        ),
        |(label, _, label_list)| Ok::<_, ErrorKind>((label, label_list)),
    )
    .parse(line)
}

// fn parse_device_label(input: &str) -> IResult<&str, String> {
//     map_res(
//         alpha1,
//         |s: &str| Ok::<_, ErrorKind>(s.to_owned()),
//     ).parse(input)
// }

fn parse_device_label(input: &str) -> IResult<&str, &str> {
    alpha1.parse(input)
}
