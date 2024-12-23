const MAGIC_PRUNE_NUMBER: u64 = 16777216; // 2 << 24

#[derive(Debug)]
pub struct Buyer {
    pub secret: u64,
}

impl Buyer {
    pub fn new(initial_secret: u64) -> Self {
        Buyer {
            secret: initial_secret,
        }
    }

    pub fn get_buying_price(&self) -> u64 {
        self.secret % 10
    }

    pub fn get_secret_after(&mut self, generation_number: usize) -> u64 {
        for _ in 0..generation_number {
            self.generate_new_secret();
        }

        self.secret
    }

    fn generate_new_secret(&mut self) {
        // Step 1
        let tmp: u64 = self.secret * 64;
        self.mix(tmp);
        self.prune();

        // Step 2
        let tmp: u64 = ((self.secret as f64) / 32_f64).floor() as u64;
        self.mix(tmp);
        self.prune();

        // Step 3
        let tmp: u64 = self.secret * 2048;
        self.mix(tmp);
        self.prune();
    }

    // Special operations

    /// "Mix" (XOR) the current secret with the given value.
    ///
    /// Examples
    /// ```
    /// # use aoc24::day22::model::Buyer;
    /// let mut buyer = Buyer::new(42);
    ///
    /// //buyer.mix(15);
    /// //assert_eq!(buyer.secret, 37);
    /// ```
    fn mix(&mut self, value: u64) {
        self.secret ^= value;
    }

    /// "Prune" (reduce modulo MAGIC_PRUNE_NUMBER) the current secret.
    ///
    /// Examples
    /// ```
    /// # use aoc24::day22::model::Buyer;
    /// let mut buyer = Buyer::new(100000000);
    ///
    /// //buyer.prune();
    /// //assert_eq!(buyer.secret, 16113920);
    /// ```
    fn prune(&mut self) {
        self.secret %= MAGIC_PRUNE_NUMBER;
    }
}

/// Will buy at price found AFTER its registered sequence
#[derive(Debug)]
pub struct Monkey {
    price_change_sequence: [i64; 4],
    pub collected_bananas: u64,
}

impl Monkey {
    pub fn new(sequence: [i64; 4]) -> Self {
        Self {
            price_change_sequence: sequence,
            collected_bananas: 0,
        }
    }

    pub fn try_to_buy_from(&mut self, prices: &[u64], price_variations: &[i64]) {
        let max_search_index: usize = price_variations.len() - 5;

        for i in 0..max_search_index {
            if price_variations[i] == self.price_change_sequence[0]
                && price_variations[i + 1] == self.price_change_sequence[1]
                && price_variations[i + 2] == self.price_change_sequence[2]
                && price_variations[i + 3] == self.price_change_sequence[3]
            {
                // println!("Prices: ...{:?}...", &prices[i-5..i+5]);
                // println!("Variat: ...{:?}...", &price_variations[i-5..i+5]);
                // println!("Collecting {} bananas", prices[i+3]);

                self.collected_bananas += prices[i + 4];
                break;
            }
        }
    }
}
