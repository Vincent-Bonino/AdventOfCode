use hashbrown::HashMap;

pub const CAPACITY: usize = 3400;

pub fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::with_capacity(CAPACITY);

    for line in input.lines() {
        let nodes: Vec<&str> = line.split("-").collect();
        let left: String = nodes[0].to_string();
        let right: String = nodes[1].to_string();

        graph.entry(left.clone()).or_default().push(right.clone());
        graph.entry(right).or_default().push(left);
    }

    graph
}
