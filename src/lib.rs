pub struct Rand {
    seed: u64,
}
impl Rand {
    /// Creates instance of random number generator.
    /// ```
    /// use simple_rnd::Rand;
    /// 
    /// let mut rand = Rand::new(0);
    /// println!("{}", rand.next());
    /// ```
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
    /// Generates the next random **u64** number.
    /// ```
    /// use simple_rnd::Rand;
    /// 
    /// let mut rand = Rand::new(0);
    /// println!("{}", rand.next());
    /// ```
    pub fn next(&mut self) -> u64 {
        // Funny enough that all numbers end with 69.
        self.seed = (self
            .seed
            .wrapping_mul(11546410263642718669)
            .wrapping_add(1542007366999009369))
        .wrapping_shr(6)
            ^ self
                .seed
                .wrapping_mul(12994751319562203769)
                .wrapping_add(992563119310360369)
                .wrapping_shl(9);
        self.seed
    }
}
