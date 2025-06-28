use std::collections::HashSet;

pub fn parse_input(input: &str) -> (HashSet<String>, Vec<String>) {
    let mut towels: HashSet<String> = HashSet::new();
    let mut patterns: Vec<String> = Vec::with_capacity(input.len());

    for (i, line) in input.lines().enumerate() {
        match i {
            0 => towels = line.split(", ").map(|pat| pat.to_string()).collect(),
            1 => {}
            _ => patterns.push(line.to_string()),
        }
    }

    (towels, patterns)
}
