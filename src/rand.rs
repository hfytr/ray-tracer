use std::{f64::consts::TAU, u64::MAX};

#[derive(Debug)]
pub struct UniformDist {
    state: u64,
}
impl UniformDist {
    fn next(&mut self) -> f64 {
        self.state = self.state * 747796405_u64 + 2891336453_u64;
        let mut result =
            ((self.state >> ((self.state >> 28) + 4_u64)) ^ self.state) * 277803737_u64;
        result = (result >> 22) ^ result;
        return (result / MAX) as f64;
    }

    fn new(seed: u64) -> UniformDist {
        Self { state: seed }
    }
}

#[derive(Debug)]
pub struct NormalDist {
    u: Option<(f64, f64)>,
    uniform: UniformDist,
}

impl NormalDist {
    pub fn normal(&mut self) -> f64 {
        let u0 = self.uniform.next();
        let u1 = self.uniform.next();
        (-2.0 * u0.ln()).sqrt() * (TAU * u1).cos()
    }

    pub fn new(seed: u64) -> NormalDist {
        Self {
            u: None,
            uniform: UniformDist::new(seed),
        }
    }
}
