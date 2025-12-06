use super::model::Problem;
use super::parsing::parse_numbers_line;

pub fn solve_part_one(raw_numbers_lines: &[String], operators: &[char]) -> String {
    let problem_number: usize = operators.len(); // An operator per problem.

    //
    // Parsing
    //

    let numbers: Vec<Vec<u64>> = raw_numbers_lines
        .iter()
        .map(|l| parse_numbers_line(l).unwrap().1)
        .collect();

    // Create empty problems
    let problem_length: usize = numbers.len();
    let mut math_problems: Vec<Problem> = Vec::with_capacity(problem_number);

    operators.iter().for_each(|op| {
        math_problems.push(Problem {
            numbers: Vec::with_capacity(problem_length),
            operator: *op,
        })
    });

    // Fill the problems
    numbers.into_iter().for_each(|num_line| {
        num_line.into_iter().enumerate().for_each(|(index, num)| {
            math_problems[index].numbers.push(num);
        })
    });

    //
    // Solving
    //

    math_problems.iter().map(|pb| pb.solve()).sum::<u64>().to_string()
}

const PART_TWO_PROBLEM_MAX_SIZE: usize = 5;

pub fn solve_part_two(raw_numbers_lines: &[String], operators: &[char]) -> String {
    let max_line_length: usize = raw_numbers_lines.iter().map(|l| l.len()).max().unwrap();

    let mut problem_numbers: Vec<Vec<u64>> = Vec::with_capacity(max_line_length / 2 + 1);
    let char_lines: Vec<Vec<char>> = raw_numbers_lines.iter().map(|l| l.chars().collect()).collect();

    //
    // Parsing
    //

    let mut current_problem_numbers: Vec<u64> = Vec::with_capacity(PART_TWO_PROBLEM_MAX_SIZE);

    // For each column, build the integer step by step
    for char_index in 0..max_line_length {
        let mut current_number: u64 = 0;

        for chars in char_lines.iter() {
            if let Some(val) = chars.get(char_index)
                && val != &' '
            {
                current_number = 10 * current_number + (*val as u64) - 48;
            }
        }

        if current_number == 0 {
            // Found empty column, this is a separator between problems
            problem_numbers.push(current_problem_numbers);
            current_problem_numbers = Vec::with_capacity(PART_TWO_PROBLEM_MAX_SIZE);
        } else {
            current_problem_numbers.push(current_number);
        }
    }

    // Do not forget the last one
    problem_numbers.push(current_problem_numbers);

    let math_problems: Vec<Problem> = operators
        .iter()
        .zip(problem_numbers)
        .map(|(op, numbers)| Problem { numbers, operator: *op })
        .collect();

    //
    // Solving
    //

    math_problems.iter().map(|pb| pb.solve()).sum::<u64>().to_string()
}
