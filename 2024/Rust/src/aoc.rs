use std::fs;

pub trait Aoc24Solution {
    fn get_day_number(&self) -> usize;
    fn get_extra_name(&self) -> Option<&str> {
        None
    }

    // File path building

    fn build_input_path(&self) -> String {
        let day_nbr: usize = self.get_day_number();
        format!("data/inputs/day{day_nbr:0>2}.txt")
    }
    fn build_test_path(&self) -> String {
        let day_nbr: usize = self.get_day_number();
        let extra: String = match self.get_extra_name() {
            None => String::from(""),
            Some(val) => format!("-{val}"),
        };
        format!("data/tests/day{day_nbr:0>2}{extra}.txt")
    }

    fn get_data(&self, is_test: bool) -> String {
        let path: String = match is_test {
            false => self.build_input_path(),
            true => self.build_test_path(),
        };
        fs::read_to_string(path).expect("File error")
    }

    // Solution parsing

    fn solve_part_one(&mut self, _is_test: bool) -> i128 {
        -1
    }
    fn solve_part_two(&mut self, _is_test: bool) -> i128 {
        -2
    }
}
