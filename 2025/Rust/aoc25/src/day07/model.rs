use aoc25_macros::AocTileEnum;

#[derive(AocTileEnum, Clone, Copy, Default)]
pub enum Tile07 {
    #[tile(chr = '.')]
    #[default]
    Empty,
    #[tile(chr = '^')]
    Splitter,
    #[tile(chr = '|')]
    Beam,
}
