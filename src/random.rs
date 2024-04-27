use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct LCG {
    state: u64,
    a: u64,
    c: u64,
    m: u64,
}

impl LCG {
    pub fn new_with_seed(seed: u64) -> Self {
        // These values for a, c, and m are typical coefficients used in LCGs.
        LCG {
            state: seed,
            a: 1664525,
            c: 1013904223,
            m: 2u64.pow(32),
        }
    }

    pub fn new() -> Self {
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
        LCG::new_with_seed(seed)
    }

    pub fn next(&mut self) -> usize {
        self.state = (self.a.wrapping_mul(self.state).wrapping_add(self.c)) % self.m;
        self.state as usize
    }
}