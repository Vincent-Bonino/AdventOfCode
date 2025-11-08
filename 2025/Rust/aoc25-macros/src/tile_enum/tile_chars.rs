use super::char_range::CharRange;

#[derive(Clone, Copy, Debug)]
pub enum TileChars {
    Range(CharRange),
    Unique(char),
}

impl TileChars {
    pub fn has_overlap_with(&self, other: &TileChars) -> bool {
        match (self, other) {
            (Self::Unique(lhs), Self::Unique(rhs)) => lhs == rhs,
            (Self::Range(lhs), Self::Range(rhs)) => lhs.has_overlap_with(rhs),
            (Self::Range(lhs), Self::Unique(rhs)) => lhs.match_char(rhs),
            (Self::Unique(lhs), Self::Range(rhs)) => rhs.match_char(lhs),
        }
    }
}
