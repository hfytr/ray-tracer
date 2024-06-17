mod hit;
pub mod obj;

use self::obj::{ObjParser, ParseReturn};
use crate::vector::Vector3;

#[derive(Debug)]
pub struct Triangle {
    vertices: [usize; 3],
    normals: [usize; 3],
}

impl Triangle {
    fn new(vertices: [usize; 3], normals: [usize; 3]) -> Triangle {
        Triangle { vertices, normals }
    }

    fn get_vertex(&self, v: u8) -> usize {
        self.vertices[v as usize]
    }
    fn get_normal(&self, v: u8) -> usize {
        self.normals[v as usize]
    }
}

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vector3<f64>>,
    normals: Vec<Vector3<f64>>,
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new_from_file(path: &str) -> Result<Mesh, std::io::Error> {
        // no textures for now
        let ParseReturn {
            vertices,
            normals,
            textures,
            triangles,
        } = ObjParser::parse(path)?;
        Ok(Mesh {
            vertices,
            normals,
            triangles,
        })
    }

    pub fn get_triangle_vertex(&self, triangle: usize, v: u8) -> &Vector3<f64> {
        &self.vertices[self.triangles[triangle].get_vertex(v)]
    }
    pub fn get_vertex(&self, i: usize) -> &Vector3<f64> {
        &self.vertices[i]
    }
    pub fn get_normal(&self, i: usize) -> &Vector3<f64> {
        &self.normals[i]
    }
}
