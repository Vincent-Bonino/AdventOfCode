use colored::Colorize;
use std::fmt::{Display, Formatter};

use comfy_table::{Cell, CellAlignment, Color, Row};
#[cfg(feature = "benchmark")]
use num_format::{Locale, ToFormattedString};

use crate::{DAY_NUMBER, TARGET_MS};

pub struct Aoc25Result {
    pub day: usize,

    // Solutions
    pub part_one_result: Option<String>,
    pub part_two_result: Option<String>,

    // Time
    #[cfg(feature = "benchmark")]
    /// Time taken to parse the input file (in μs)
    pub parsing_duration: u128,
    #[cfg(feature = "benchmark")]
    /// Time taken to solve the first part of the puzzle (in μs).
    pub part_one_duration: u128,
    #[cfg(feature = "benchmark")]
    /// Time taken to solve the first part of the puzzle (in μs).
    pub part_two_duration: u128,
}

impl Aoc25Result {
    #[cfg(feature = "benchmark")]
    pub fn get_total_duration(&self) -> u128 {
        self.parsing_duration + self.part_one_duration + self.part_two_duration
    }

    #[cfg(feature = "benchmark")]
    pub fn duration_to_cell(duration: u128, total: bool) -> Cell {
        let cell = Cell::new(duration.to_string()).set_alignment(CellAlignment::Right);

        if total && duration > TARGET_MS / DAY_NUMBER as u128 {
            cell.fg(Color::Red)
        } else {
            cell
        }
    }

    pub fn get_part_one_string(&self) -> String {
        match &self.part_one_result {
            Some(result) => format!("'{result}'"),
            None => String::from("-"),
        }
    }

    pub fn get_part_two_string(&self) -> String {
        match &self.part_two_result {
            Some(result) => format!("'{result}'"),
            None => String::from("-"),
        }
    }
}

impl Display for Aoc25Result {
    #[cfg(not(feature = "benchmark"))]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let day_str: String = format!("[Day {:0>2}]", self.day);

        match (&self.part_one_result, &self.part_two_result) {
            (None, _) => {
                write!(f, "{day_str} - No result for the moment")
            }
            (Some(part_one), None) => {
                write!(f, "{day_str}\nPart 1: '{part_one}'")
            }
            (Some(part_one), Some(part_two)) => {
                write!(f, "{day_str}\nPart 1: '{part_one}'\nPart 2: '{part_two}'")
            }
        }
    }

    #[cfg(feature = "benchmark")]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let day_str: String = format!("[Day {:0>2}]", self.day);

        match (&self.part_one_result, &self.part_two_result) {
            (None, _) => {
                write!(f, "{day_str} - No result for the moment")
            }
            (Some(part_one), None) => {
                write!(
                    f,
                    "{day_str}\nParsing: {}\nPart 1: '{part_one}' in {}",
                    format_duration(self.parsing_duration),
                    format_duration(self.part_one_duration),
                )
            }
            (Some(part_one), Some(part_two)) => {
                write!(
                    f,
                    "{day_str}\nParsing: {}\nPart 1: '{part_one}' in {}\nPart 2: '{part_two}' in {}\nTotal: {}",
                    format_duration(self.parsing_duration),
                    format_duration(self.part_one_duration),
                    format_duration(self.part_two_duration),
                    format_duration(self.get_total_duration()),
                )
            }
        }
    }
}

impl From<Aoc25Result> for Row {
    fn from(val: Aoc25Result) -> Self {
        let mut row = Row::new();
        row.add_cell(val.day.into())
            .add_cell(val.get_part_one_string().into())
            .add_cell(val.get_part_two_string().into());

        #[cfg(feature = "benchmark")]
        row.add_cell(Aoc25Result::duration_to_cell(val.parsing_duration, false))
            .add_cell(Aoc25Result::duration_to_cell(val.part_one_duration, false))
            .add_cell(Aoc25Result::duration_to_cell(val.part_two_duration, false))
            .add_cell(Aoc25Result::duration_to_cell(val.get_total_duration(), false));

        row
    }
}

#[cfg(feature = "benchmark")]
pub fn format_duration(micros: u128) -> String {
    let formated_micros: String = micros.to_formatted_string(&Locale::fr);

    if micros < 1_000 {
        format!("{formated_micros}μs")
    } else if micros < 1_000_000 {
        let formatted_millis: String = (micros / 1000).to_formatted_string(&Locale::fr);
        format!("{formatted_millis}ms ({formated_micros}μs)")
    } else if micros < 1_000_000_000 {
        let formatted_secs: String = (micros / 1_000_000).to_formatted_string(&Locale::fr);
        format!("{formatted_secs}s ({formated_micros}μs)")
    } else {
        format!("too long ({formated_micros}μs)")
    }
}
