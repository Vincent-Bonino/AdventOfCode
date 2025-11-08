use aoc25_macros::AocTileEnum;

#[derive(AocTileEnum, Clone, Default, Debug)]
pub enum Tile00 {
    #[tile(chr = '#')]
    Wall,
    #[default]
    #[chr = '.']
    Floor,
    #[chr = '~']
    Water,
}
