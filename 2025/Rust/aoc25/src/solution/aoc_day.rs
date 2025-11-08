/// Trait containing a single function, give the day of the puzzle.
///
/// Separated from [`Aoc25Solution`](crate::solution::Aoc25Solution) for practical reason:
/// since multiple block trait implementation is not possible, it was made
/// its own trait to be able to generate the impl block from a derive macro.
pub trait Aoc25Day {
    /// Return the numeric value of the day of the puzzle.
    fn get_day_number(&self) -> usize;
}
