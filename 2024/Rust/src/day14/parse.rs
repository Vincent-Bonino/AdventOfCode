use crate::day14::model::SecurityRobot;
use crate::toolbox::parsing::parse_numbers;

pub fn parser_input(input: &str) -> Vec<SecurityRobot> {
    input
        .lines()
        .map(|line| {
            let numbers = parse_numbers(line);
            SecurityRobot::new(numbers[0], numbers[1], numbers[2], numbers[3])
        })
        .collect()
}
