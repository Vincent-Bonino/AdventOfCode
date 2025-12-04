use aoc25_macros::AocTileEnum;

// Only used for easy to write parsing.
#[derive(AocTileEnum, Clone, Copy, Default)]
pub enum Tile04 {
    #[default]
    #[chr = '.']
    Empty,
    #[chr = '@']
    PaperRoll,
}
