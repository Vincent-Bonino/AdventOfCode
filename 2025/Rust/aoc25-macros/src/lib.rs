// -- Sub-modules declarations --
mod day;
mod tile_enum;
mod utils;

// -- Imports --
use proc_macro::TokenStream;

#[proc_macro_derive(AocTileEnum, attributes(chr, range, tile))]
/// Derive macro for AdventOfCode "tile" enumerations.
///
/// Are currently implemented:
///  - [`Display`](std::fmt::Display)
///  - [`TryFrom<char>`]
///
/// Works in conjunction with the `tile` attribute.
///
/// ## Examples
///
/// Full example:
/// ```rust
/// # use aoc25_macros::AocTileEnum;
/// #[derive(AocTileEnum)]
/// enum TileExample {
///     #[tile(chr = '.')]
///     Floor,
///     #[tile(chr = '#')]
///     Wall,
///     #[tile(range = "digit")]
///     City(char),
/// }
/// ```
///
/// The `chr` and `range` attributes are also accepted.
///
/// So the following example is similar to the previous one:
/// ```rust
/// # use aoc25_macros::AocTileEnum;
/// #[derive(AocTileEnum)]
/// enum TileExample {
///     #[chr = '.']
///     Floor,
///     #[chr = '#']
///     Wall,
///     #[range = "digit"]
///     City(char),
/// }
/// ```
///
/// Other attributes are preserved, allowing for instance to add `#[default]` to any variant.
pub fn aoc_tile_enum(input: TokenStream) -> TokenStream {
    tile_enum::aoc_tile_enum(input)
}

#[proc_macro_derive(Aoc25Day)]
/// Derive macro for to extract day number from AdventOfCode solutions.
///
/// **Note:** day number is extracted from the two last characters of the name.
///
/// ## Example
///
/// ```ignore
/// # use aoc25_macros::Aoc25Day;
/// #[derive(Aoc25Day)]
/// struct Day00 { /* ... */ }
///
/// let d = Day00{};
/// assert_eq!(0, d.get_day_number())
/// ```
pub fn aoc_day_struct(input: TokenStream) -> TokenStream {
    day::aoc_day_struct(input)
}
