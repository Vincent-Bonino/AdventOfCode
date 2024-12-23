pub fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|val| val.parse().unwrap()).collect()
}
