const MIN_DELTA: i32 = 1;
const MAX_DELTA: i32 = 3;

pub fn solve_part_one(reports: &[Vec<i32>]) -> i128 {
    reports
        .iter()
        .map(|report| match is_safe(report) {
            true => 1,
            false => 0,
        })
        .sum::<i128>()
}

fn is_safe(report: &[i32]) -> bool {
    let is_increasing: bool = report.get(1).unwrap() >= report.first().unwrap();

    for index in 1..report.len() {
        let val1: &i32 = report.get(index - 1).unwrap();
        let val2: &i32 = report.get(index).unwrap();

        if (val2 >= val1) != is_increasing {
            return false;
        }

        let delta: i32 = (val2 - val1).abs();
        if !(MIN_DELTA..=MAX_DELTA).contains(&delta) {
            return false;
        }
    }

    true
}

pub fn solve_part_two(reports: &[Vec<i32>]) -> i128 {
    reports
        .iter()
        .map(|report| match is_safe(report) || is_pseudo_safe(report) {
            true => 1,
            false => 0,
        })
        .sum::<i128>()
}

// Brute-force sub-report safety check, cost is low
fn is_pseudo_safe(report: &[i32]) -> bool {
    for index in 0..report.len() {
        let mut subreport: Vec<i32> = report.to_owned();
        subreport.remove(index);

        if is_safe(&subreport) {
            return true;
        }
    }
    false
}
