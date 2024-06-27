pub mod mesh;

use crate::obj::ObjParser;
use crate::{ray::Ray, vector::Vector3};
use mesh::*;
use std::f64::EPSILON;
use std::ops::Range;

#[derive(Debug, Default)]
pub struct Scene {
    pub vertices: Vec<Vector3<f64>>,
    pub point_normals: Vec<Vector3<f64>>,
    pub face_normals: Vec<Vector3<f64>>,
    pub triangles: Vec<Triangle>,
    pub materials: Vec<Material>,
    pub meshes: Vec<usize>,
}

impl Scene {
    pub fn from_obj(path: &str) -> Result<Scene, std::io::Error> {
        let mut parser = ObjParser::default();
        parser.parse(path)?;
        let ObjParser {
            point_normals,
            face_normals,
            vertices,
            triangles,
            materials,
            meshes,
            ..
        } = parser;
        Ok(Self {
            point_normals,
            face_normals,
            vertices,
            triangles,
            materials,
            meshes,
        })
    }

    pub fn hits(&self, ray: &Ray, acne_threshold: f64) -> Option<(f64, usize)> {
        let mut result = None;
        for i in 0..self.meshes.len() {
            if let Some((t, triangle)) = self.hits_mesh(i, &ray, acne_threshold) {
                result = match result {
                    None => Some((t, triangle)),
                    Some((prev_t, _)) => {
                        if prev_t < t {
                            result
                        } else {
                            Some((t, triangle))
                        }
                    }
                };
            }
        }
        result
    }

    // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    fn hits_mesh(&self, mesh: usize, ray: &Ray, acne_threshold: f64) -> Option<(f64, usize)> {
        let mut result = None;
        for triangle in self.get_triangle_range(mesh) {
            let v0 = self.get_triangle_vertex(triangle, 0);
            let v1 = self.get_triangle_vertex(triangle, 1);
            let v2 = self.get_triangle_vertex(triangle, 2);

            let e0 = v1 - v0;
            let e1 = v2 - v0;

            let ray_x_e1 = ray.dir().cross(&e1);
            let determinant = ray_x_e1.dot(&e0);
            if determinant.abs() < EPSILON {
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
            if t > acne_threshold {
                result = match result {
                    None => Some((t, triangle)),
                    Some((prev_t, _)) => {
                        if prev_t < t {
                            result
                        } else {
                            Some((t, triangle))
                        }
                    }
                }
            }
        }
        result
    }

    fn get_triangle_range(&self, mesh: usize) -> Range<usize> {
        let lower = self.meshes[mesh];
        let upper = if mesh == self.meshes.len() - 1 {
            self.triangles.len()
        } else {
            self.meshes[mesh + 1]
        };
        lower..upper
    }

    pub fn get_triangle_vertex(&self, triangle: usize, v: u8) -> &Vector3<f64> {
        &self.vertices[self.triangles[triangle].vertices[v as usize]]
    }
    pub fn get_triangel_mat(&self, triangle: usize) -> &Material {
        &self.materials[self.triangles[triangle].material]
    }
    pub fn get_face_normal(&self, triangle: usize) -> &Vector3<f64> {
        &self.face_normals[self.triangles[triangle].face_normal]
    }
}
