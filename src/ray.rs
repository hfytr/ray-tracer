use crate::vector::Vector3;

#[derive(Debug)]
pub struct Ray {
    orig: Vector3<f64>,
    dir: Vector3<f64>,
}

impl Ray {
    pub fn new(mut orig: Vector3<f64>, dir: Vector3<f64>) -> Ray {
        orig.normalize();
        Ray { orig, dir }
    }

    pub fn dir(&self) -> &Vector3<f64> {
        &self.dir
    }

    pub fn orig(&self) -> &Vector3<f64> {
        &self.orig
    }
}
