use once_cell::sync::Lazy;
use regex::Regex;

static NUMBERS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"-?\d+").unwrap());

pub fn parse_numbers(line: &str) -> Vec<i32> {
    NUMBERS_REGEX
        .find_iter(line)
        .map(|re_match| re_match.as_str().parse::<i32>().unwrap())
        .collect()
}
