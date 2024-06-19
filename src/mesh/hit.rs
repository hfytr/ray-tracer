use std::f64::EPSILON;

use crate::{mesh::Mesh, ray::Ray, vector::Vector3};

impl Mesh {
    // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    pub fn hits(&self, ray: &Ray) -> Option<Vector3<f64>> {
        for t in 0..self.triangles.len() {
            let v0 = self.get_triangle_vertex(t, 0);
            let v1 = self.get_triangle_vertex(t, 1);
            let v2 = self.get_triangle_vertex(t, 2);

            let e0 = v1 - v0;
            let e1 = v2 - v0;

            let ray_x_e1 = ray.dir().cross(&e1);
            let determinant = ray_x_e1.dot(&e0);
            if determinant.abs() < EPSILON {
                print!("a");
                continue;
            }

            let inverse_det = 1.0 / determinant;
            let a = ray.orig() - v0;
            let u = inverse_det * a.dot(&ray_x_e1);
            if u < 0.0 || u > 1.0 {
                continue;
            }

            let a_x_e0 = a.cross(&e0);
            let v = inverse_det * ray.dir().dot(&a_x_e0);
            if v < 0.0 || u + v > 1.0 {
                continue;
            }

            let t = inverse_det * e1.dot(&a_x_e0);
            if t > EPSILON {
                return Some(ray.dir() * t + ray.orig());
            }
        }
        None
    }
}
