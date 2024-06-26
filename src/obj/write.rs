use std::{fs::File, io::Write};

use crate::scene::{mesh::Triangle, Scene};
use crate::{ray::Ray, vector::Vector3};

const PIXELS_PER_METER: f64 = 100.0;
#[derive(Debug, Default)]
pub struct ObjWriter {
    vertices: Vec<String>,
    normals: Vec<String>,
    faces: Vec<String>,
    lines: Vec<String>,
}

impl ObjWriter {
    pub fn new() -> ObjWriter {
        Self::default()
    }

    pub fn write(&self, path: &str) -> Result<(), std::io::Error> {
        let mut text = String::new();
        for v in self.vertices.iter() {
            text.push_str(&format!("v {}", v));
            text.push('\n');
        }
        for n in self.normals.iter() {
            text.push_str(&format!("vn {}", n));
            text.push('\n');
        }
        for l in self.lines.iter() {
            text.push_str(&format!("l {}", l));
            text.push('\n');
        }
        for f in self.faces.iter() {
            text.push_str(&format!("f {}", f));
            text.push('\n');
        }

        let mut file = File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;
        file.write_all(text.as_bytes())
    }

    pub fn add_vertex(&mut self, v: &Vector3<f64>) {
        self.vertices.push(self.write_vector(v));
    }
    pub fn add_normal(&mut self, n: &Vector3<f64>) {
        self.normals.push(self.write_vector(n));
    }
    pub fn add_triangle(&mut self, t: &Triangle) {
        self.faces.push(self.write_triangle(t));
    }
    pub fn add_ray(&mut self, r: &Ray, t: f64) {
        self.add_vertex(r.orig());
        self.add_vertex(&(r.orig() + r.dir() * t));
        let num_vertices = self.vertices.len();
        let text = format!("{} {}", num_vertices + 1, num_vertices + 2);
        self.lines.push(text);
    }

    fn write_triangle(&self, t: &Triangle) -> String {
        let mut text = String::new();
        for i in 0..3 {
            text.push_str(&format!(
                "{}//{} ",
                t.vertices[i] + 1,
                t.point_normals[i] + 1
            ));
        }
        text
    }

    fn write_vector(&self, v: &Vector3<f64>) -> String {
        format!(
            "{} {} {}",
            v[0] / PIXELS_PER_METER,
            v[1] / PIXELS_PER_METER,
            v[2] / PIXELS_PER_METER
        )
    }

    pub fn add_scene(&mut self, scene: &Scene, face_normals: bool) {
        for v in scene.vertices.iter() {
            self.add_vertex(v);
        }
        for n in scene.point_normals.iter() {
            self.add_normal(n);
        }
        for t in scene.triangles.iter() {
            self.add_triangle(t);
        }
        if face_normals {
            for triangle in scene.triangles.iter() {
                let middleish = triangle
                    .vertices
                    .iter()
                    .map(|i| &scene.vertices[*i])
                    .fold(Vector3::<f64>::default(), |acc, elem| acc + elem)
                    * (1.0 / 3.0);
                let r = Ray::new(middleish, scene.face_normals[triangle.face_normal].clone());
                self.add_ray(&r, 100.0);
            }
        }
    }
}
