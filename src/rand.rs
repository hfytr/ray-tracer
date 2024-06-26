use std::{f64::consts::TAU, num::Wrapping};

const MAX: f64 = std::u64::MAX as f64;

#[inline]
fn rotl(x: u64, k: i32) -> u64 {
    (x << k) | (x >> (64 - k))
}

#[derive(Debug)]
pub struct UniformDist([u64; 2]);
impl UniformDist {
    fn next(&mut self) -> u64 {
        let s0 = Wrapping(self.0[0]);
        let mut s1 = Wrapping(self.0[1]);
        let result = s0 + s1;
        s1 ^= s0;
        self.0[0] = (Wrapping(rotl(s0.0, 55)) ^ s1 ^ (s1 << 14)).0;
        self.0[1] = rotl(s1.0, 36);
        result.0
    }

    fn new(seed: [u64; 2]) -> UniformDist {
        Self([seed[0], seed[1]])
    }
}

#[derive(Debug)]
pub struct NormalDist {
    u: Option<(f64, f64)>,
    uniform: UniformDist,
}

impl NormalDist {
    pub fn normal(&mut self) -> f64 {
        let u0 = self.uniform.next() as f64 / MAX;
        let u1 = self.uniform.next() as f64 / MAX;
        let got = (-2.0 * u0.ln()).sqrt() * (TAU * u1).cos();
        got
    }

    pub fn new(seed: [u64; 2]) -> NormalDist {
        Self {
            u: None,
            uniform: UniformDist::new([seed[0], seed[1]]),
        }
    }
}
