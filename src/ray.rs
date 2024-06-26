use crate::{rand::NormalDist, vector::Vector3};

#[derive(Debug, Default)]
pub struct Ray {
    orig: Vector3<f64>,
    dir: Vector3<f64>,
}

impl Ray {
    pub fn new(orig: Vector3<f64>, dir: Vector3<f64>) -> Ray {
        Ray { orig, dir }
    }

    pub fn dir(&self) -> &Vector3<f64> {
        &self.dir
    }

    pub fn orig(&self) -> &Vector3<f64> {
        &self.orig
    }

    pub fn rand_dir(random_state: &mut NormalDist) -> Vector3<f64> {
        Vector3::new(
            random_state.normal(),
            random_state.normal(),
            random_state.normal(),
        )
    }

    pub fn point_at(&self, t: f64) -> Vector3<f64> {
        &self.orig + &self.dir * t
    }
}
