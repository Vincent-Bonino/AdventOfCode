/// FreshRanges are inclusive.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FreshRange {
    pub start: u64,
    pub end: u64,
}

impl FreshRange {
    #[inline]
    pub fn contains(&self, val: u64) -> bool {
        self.start <= val && val <= self.end
    }

    #[inline]
    pub fn len(&self) -> u64 {
        self.end - self.start + 1
    }

    /// Merge with another range.
    ///
    /// # Returns
    /// `true` if they could be merged, in which case `self` is modified.
    /// `false` if they couldn't.
    pub fn try_merge_with(&mut self, other: &FreshRange) -> bool {
        if self.start <= other.start && other.end <= self.end {
            // Other is included in self
            true
        } else if other.start <= self.start && self.end <= other.end {
            // Self is included in other
            self.start = other.start;
            self.end = other.end;
            true
        } else if self.contains(other.start) || self.contains(other.end) {
            // Partially overlap
            self.start = self.start.min(other.start);
            self.end = other.end.max(other.end);
            true
        } else {
            // Completely disjoined
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day05::model::FreshRange;

    #[test]
    fn test_len() {
        assert_eq!(FreshRange { start: 5, end: 10 }.len(), 6);
    }
}
