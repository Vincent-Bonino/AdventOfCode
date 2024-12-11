const YEAR: u128 = 2024;

#[derive(Copy, Clone, Debug)]
pub struct Stone {
    pub value: u128,
    pub multiplier: u128,
}

impl Stone {
    pub fn new(value: u128) -> Self {
        Self {
            value,
            multiplier: 1,
        }
    }

    pub fn with_value(&self, value: u128) -> Self {
        Self {
            value,
            multiplier: self.multiplier,
        }
    }

    /// Apply the first applying rule
    pub fn change(&self) -> Vec<Self> {
        // First rule, '0' stones become '1'
        if self.value == 0 {
            return vec![self.with_value(1)];
        }

        // Second rule, even length number stones are split in two
        if (self.value.ilog10() + 1) % 2 == 0 {
            return self.split();
        }

        // Default rule, multiply the value by the year
        vec![self.with_value(self.value * YEAR)]
    }

    #[inline]
    /// Split a stone in two.
    ///
    /// Example:
    /// Stone{XXXYYY} => Stone{XXX} and Stone{YYY}
    fn split(&self) -> Vec<Self> {
        let half_length: u32 = (self.value.ilog10() + 1) / 2;

        vec![
            self.with_value(self.value / (10_u128.pow(half_length))),
            self.with_value(self.value % (10_u128.pow(half_length))),
        ]
    }
}
