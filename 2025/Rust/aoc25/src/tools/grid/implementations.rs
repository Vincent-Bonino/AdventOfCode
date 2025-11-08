use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use colored::{ColoredString, Colorize};

use super::Grid;
use crate::coord;
use crate::tools::coordinates::Coordinates;
use crate::tools::types::CoordIntType;

impl<T: TryFrom<char>> FromStr for Grid<T>
where
    <T as TryFrom<char>>::Error: Debug,
{
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_nested(
            value
                .lines()
                .map(|line| line.chars().map(|c| c.try_into().unwrap()).collect())
                .collect(),
        ))
    }
}

impl<T: Clone> Grid<T> {
    pub fn fill(value: T, rows: usize, cols: usize) -> Self {
        Self {
            data: vec![value; rows * cols],
            rows,
            cols,
        }
    }
}

impl<T: Default + TryFrom<char>> Grid<T> {
    /// Build a grid from the provided string.
    ///
    /// Provided `allowed_markers` will be replaced by `T::default`,
    /// and their coordinates will be returned with the built grid as a [`HashMap`].
    pub fn from_str_with_markers(
        value: &str,
        allowed_markers: &[char],
    ) -> Result<(Self, HashMap<char, Coordinates>), String> {
        let mut unknown_chars: HashMap<char, Vec<Coordinates>> = HashMap::new();

        let mut markers: HashMap<char, Coordinates> = HashMap::new();
        let grid: Self = Self::from_nested(
            value
                .lines()
                .enumerate()
                .map(|(line_idx, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(chr_index, chr)| match chr.try_into() {
                            Ok(t) => t,
                            Err(_) => {
                                if allowed_markers.contains(&chr) {
                                    markers.insert(chr, coord!(line_idx, chr_index));
                                } else {
                                    unknown_chars.entry(chr).or_default().push(coord!(line_idx, chr_index));
                                }
                                T::default()
                            }
                        })
                        .collect()
                })
                .collect(),
        );

        if unknown_chars.is_empty() {
            Ok((grid, markers))
        } else {
            Err(unknown_chars
                .into_iter()
                .map(|(chr, coords)| format!("Unknown char '{chr}' at {coords:?}"))
                .collect::<Vec<String>>()
                .join("; "))
        }
    }
}

// Display of the grid

const MARK_CHAR: char = 'X';
const PATH_CHAR: char = 'o';
const MARK_AND_PATH_CHAR: char = '+';

const SIDE_COORD_PADDING: usize = 3;

impl<T: Display> Grid<T> {
    pub fn show(&self) {
        self._show(None, None, None, None, None, None, None, None)
    }

    /// Display the grid with a marked coordinates.
    ///
    /// The `mark_char` parameter:
    ///     - None will display the value found in the grid
    ///     - Some(None) will display a default value
    ///     - Some(Some(char)) will display the provided char
    pub fn show_with_mark(&self, mark_coordinates: &Coordinates, mark_char: Option<Option<char>>) {
        self._show(None, None, None, None, None, None, Some(mark_coordinates), mark_char)
    }

    /// Display the grid with a path of coordinates.
    ///
    /// The `path_char` parameter:
    ///     - None will display the value found in the grid
    ///     - Some(None) will display a default value
    ///     - Some(Some(char)) will display the provided char
    pub fn show_with_path(&self, path_coordinates: &[Coordinates], path_char: Option<Option<char>>) {
        self._show(None, None, None, None, Some(path_coordinates), path_char, None, None)
    }

    #[allow(clippy::too_many_arguments)]
    /// Base function for showing a Grid.
    ///
    /// Notes:
    ///  - `max_row` and `max_col` parameters are excluded (add 1 if you want them showed).
    ///  - `mark_char` and `path_char` parameters are `Option<Option<char>>`:
    ///     - None will display the value found in the grid
    ///     - Some(None) will display a default value
    ///     - Some(Some(char)) will display the provided char
    fn _show(
        &self,
        min_row: Option<usize>,
        max_row: Option<usize>,
        min_col: Option<usize>,
        max_col: Option<usize>,
        path: Option<&[Coordinates]>,
        path_char: Option<Option<char>>,
        mark: Option<&Coordinates>,
        mark_char: Option<Option<char>>,
    ) {
        // -- Handle arguments --
        if path.is_none() && path_char.is_some() {
            eprintln!("Error showing Grid, got `path_char` but no `path`");
            return;
        } else if mark.is_none() && mark_char.is_some() {
            eprintln!("Error showing Grid, got `mark_char` but no `mark`");
            return;
        }

        // -- Set default arguments
        let min_row: usize = min_row.unwrap_or(0);
        let max_row: usize = max_row.unwrap_or(self.rows);
        let min_col: usize = min_col.unwrap_or(0);
        let max_col: usize = max_col.unwrap_or(self.cols);

        // Header
        let header_footer: String = self.build_header_footer(min_col, max_col - min_col);
        println!("{}", header_footer.cyan().italic());

        // Grid
        for row in min_row..max_row {
            let row_string = row.to_string().underline();
            let padding_string = self.build_padding_string(' ', SIDE_COORD_PADDING - row_string.len());

            // Left index
            print!("{}{}  ", padding_string, row_string.clone().cyan()); // SIDE_COORD_PADDING; 2 spaces

            for col in min_col..max_col {
                let coord = coord!(row, col);
                let value: &T = self.get_ref(&coord).expect("Out of bounds");

                let is_in_mark: bool = mark.is_some_and(|mark| *mark == coord);
                let is_in_path: bool = path.unwrap_or_default().contains(&coord);

                match (is_in_mark, is_in_path) {
                    (true, true) => print!("{}", self.decide_mark_and_path_value(mark_char, path_char, value)),
                    (true, false) => print!("{}", self.decide_mark_value(mark_char, value)),
                    (false, true) => print!("{}", self.decide_path_value(path_char, value)),
                    (false, false) => print!("{value}"),
                }
            }

            // Right index
            print!("  {}{}", padding_string, row_string.cyan()); // 2 spaces; 3 = SIDE_COORD_PADDING

            // New line
            println!();
        }

        // Footer
        println!("{}\n", header_footer.cyan().italic());
    }

    // Utils

    fn build_padding_string(&self, value: char, length: usize) -> String {
        let mut result: String = String::new();
        for _ in 0..length {
            result.push(value);
        }
        result
    }

    /// Build the index line used as a header/footer when showing the grid.
    fn build_header_footer(&self, start_col: usize, col_number: usize) -> String {
        // Number of columns + for each side (2 spaces + SIDE_COORD_PADDING)
        let mut result: String = String::with_capacity(col_number + 2 * (2 + SIDE_COORD_PADDING));

        // Left padding
        for _ in 0..(2 + SIDE_COORD_PADDING) {
            result.push(' ');
        }

        // Indexes
        for i in start_col..col_number {
            result.push_str(&(i % 10).to_string()); // str will be 1 char long
        }

        result
    }

    /// Decide what to print on marked coordinates.
    ///  - None will display the value found in the grid
    ///  - Some(None) will display a default value
    ///  - Some(Some(char)) will display the provided char
    fn decide_mark_value(&self, rule: Option<Option<char>>, value: &T) -> impl Display {
        match rule {
            None => value.to_string().red(),
            Some(None) => MARK_CHAR.to_string().red(),
            Some(Some(mark_char)) => mark_char.to_string().red(),
        }
    }

    /// Decide what to print on path coordinates.
    ///  - None will display the value found in the grid
    ///  - Some(None) will display a default value
    ///  - Some(Some(char)) will display the provided char
    fn decide_path_value(&self, rule: Option<Option<char>>, value: &T) -> impl Display {
        match rule {
            None => value.to_string().cyan(),
            Some(None) => PATH_CHAR.to_string().cyan(),
            Some(Some(path_char)) => path_char.to_string().cyan(),
        }
    }

    /// Decide what to print on mark and path coordinates.
    ///  - (None, None) will display the value found in the grid
    ///  - (Some(Some(char)), _) will display the provided char
    ///  - (_, Some(Some(char))) will display the provided char
    ///  - Anything else will display a default value
    fn decide_mark_and_path_value(
        &self,
        mark_rule: Option<Option<char>>,
        path_rule: Option<Option<char>>,
        value: &T,
    ) -> impl Display {
        match (mark_rule, path_rule) {
            (None, None) => value.to_string().yellow(),
            (Some(Some(mark_char)), _) => mark_char.to_string().yellow(),
            (_, Some(Some(path_char))) => path_char.to_string().yellow(),
            _ => MARK_AND_PATH_CHAR.to_string().yellow(),
        }
    }
}
