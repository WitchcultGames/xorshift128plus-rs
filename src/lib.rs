use std::num::Wrapping;

pub struct XorShift128Plus {
    state: [Wrapping<u64>; 2],
    seed: Wrapping<u64>,
}

impl XorShift128Plus {
    pub fn new(seed: u64) -> XorShift128Plus {
        let mut new = XorShift128Plus {
            state: [Wrapping(0), Wrapping(0)],
            seed: Wrapping(0),
        };

        new.seed(seed);

        new
    }

    pub fn seed(&mut self, seed: u64) {
        self.seed = Wrapping(seed);
        self.state[0] = Wrapping(1812433253) * (self.seed ^ (self.seed >> 30)) + Wrapping(1);
        self.state[1] = Wrapping(1812433253)
                        * (self.state[0] ^ (self.state[0] >> 30))
                        + Wrapping(2);
    }

    fn next(&mut self) -> u64 {
        let x = self.state[0] ^ self.state[0] << 23;
        let y = self.state[1];
        self.state[0] = y;
        self.state[1] = x ^ y ^ (x >> 18) ^ (y >> 5);
        (self.state[1] + y).0
    }

    pub fn chance(&mut self, chance: f32) -> bool {
        chance >= self.random_factor()
    }

    pub fn random_factor(&mut self) -> f32 {
        self.next() as f32 / std::u64::MAX as f32
    }

    pub fn range<T: XorShift128PlusRandomizable>(&mut self, minimum: T, maximum: T) -> T {
        T::random_range(minimum, maximum, self)
    }
}

pub trait XorShift128PlusRandomizable {
    fn random_range(minimum: Self, maximum: Self, prng: &mut XorShift128Plus) -> Self;
}

impl XorShift128PlusRandomizable for u8 {
    fn random_range(minimum: Self, maximum: Self, prng: &mut XorShift128Plus) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl XorShift128PlusRandomizable for u16 {
    fn random_range(minimum: Self, maximum: Self, prng: &mut XorShift128Plus) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl XorShift128PlusRandomizable for u32 {
    fn random_range(minimum: Self, maximum: Self, prng: &mut XorShift128Plus) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl XorShift128PlusRandomizable for u64 {
    fn random_range(minimum: Self, maximum: Self, prng: &mut XorShift128Plus) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl XorShift128PlusRandomizable for usize {
    fn random_range(minimum: Self, maximum: Self, prng: &mut XorShift128Plus) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl XorShift128PlusRandomizable for i8 {
    fn random_range(minimum: Self, maximum: Self, prng: &mut XorShift128Plus) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl XorShift128PlusRandomizable for i16 {
    fn random_range(minimum: Self, maximum: Self, prng: &mut XorShift128Plus) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl XorShift128PlusRandomizable for i32 {
    fn random_range(minimum: Self, maximum: Self, prng: &mut XorShift128Plus) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl XorShift128PlusRandomizable for i64 {
    fn random_range(minimum: Self, maximum: Self, prng: &mut XorShift128Plus) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl XorShift128PlusRandomizable for isize {
    fn random_range(minimum: Self, maximum: Self, prng: &mut XorShift128Plus) -> Self {
        let diff = maximum.wrapping_sub(minimum) as f32;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl XorShift128PlusRandomizable for f32 {
    fn random_range(minimum: Self, maximum: Self, prng: &mut XorShift128Plus) -> Self {
        let diff = (maximum - minimum) as f32;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}

impl XorShift128PlusRandomizable for f64 {
    fn random_range(minimum: Self, maximum: Self, prng: &mut XorShift128Plus) -> Self {
        let diff = (maximum - minimum) as f32;
        let min = minimum as f32;

        (min + (prng.random_factor() * diff)) as Self
    }
}
