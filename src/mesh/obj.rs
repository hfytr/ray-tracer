use core::panic;
use std::{
    fs::{read_to_string, File},
    io::Write,
    str::FromStr,
};

use super::{Mesh, Triangle};
use crate::{ray::Ray, vector::Vector3};

const PIXELS_PER_METER: f64 = 100.0;

fn collect_array<T, const N: usize>(mut i: impl Iterator<Item = T>) -> [T; N] {
    std::array::from_fn(|_| i.next().unwrap())
}

pub struct FaceEntryElement {
    pub vertex: usize,
    pub texture: Option<usize>,
    pub normal: Option<usize>,
}

impl FaceEntryElement {
    pub fn print(&self) -> String {
        let texture_string = match self.texture {
            Some(u) => u.to_string(),
            None => String::new(),
        };
        let normal_string = match self.normal {
            Some(u) => u.to_string(),
            None => String::new(),
        };
        format!("{}/{}/{}", self.vertex, texture_string, normal_string)
    }
}

impl FaceEntryElement {
    pub fn new(s: &str) -> Result<FaceEntryElement, <usize as FromStr>::Err> {
        let w: Vec<&str> = s.split('/').collect();
        Ok(FaceEntryElement {
            vertex: w[0].parse::<usize>()?,
            texture: w[1].parse::<usize>().ok(),
            normal: w[2].parse::<usize>().ok(),
        })
    }
}

pub struct ParseReturn {
    pub vertices: Vec<Vector3<f64>>,
    pub normals: Vec<Vector3<f64>>,
    pub textures: Vec<Vector3<f64>>,
    pub triangles: Vec<Triangle>,
}

pub struct ObjParser;
impl ObjParser {
    pub fn parse(path: &str) -> Result<ParseReturn, std::io::Error> {
        let file_string = read_to_string(path)?;
        println!("{}", file_string);
        let lines: Vec<Vec<&str>> = file_string
            .lines()
            .filter(|s| s.chars().next().unwrap_or('#') != '#')
            .map(|s| s.split(' ').collect())
            .collect();
        let mut normals = Vec::new();
        let mut vertices = Vec::new();
        let mut textures = Vec::new();
        let mut triangles = Vec::new();
        for mut line in lines {
            println!("{:?}", line);
            match line[0] {
                "f" => {
                    assert_eq!(4, line.len(), "this face is not a triangle, please triangulate the surface before parsing");
                    triangles.push(Self::parse_face(line).expect("normals not provided"));
                }
                "v" => {
                    vertices.push(Self::parse_vector(line));
                }
                "vt" => {
                    if line.len() < 4 && line.len() > 2 {
                        while line.len() < 4 {
                            line.push("0.0");
                        }
                    }
                    textures.push(Self::parse_vector(line));
                }
                "vn" => {
                    normals.push(Self::parse_vector(line));
                }
                _ => {}
            }
        }
        Ok(ParseReturn {
            vertices,
            normals,
            textures,
            triangles,
        })
    }

    fn parse_face(line: Vec<&str>) -> Option<Triangle> {
        assert_eq!(
            line.len(),
            4,
            "please triangulate the obj file, then try again"
        );
        let parsed: Vec<FaceEntryElement> = line
            .into_iter()
            .skip(1)
            .map(|s| FaceEntryElement::new(s).expect("invalid face element"))
            .collect();
        for p in parsed.iter() {
            if p.texture.is_some() {
                println!("texture not currently supported");
                break;
            }
        }
        Some(Triangle::new(
            collect_array(parsed.iter().map(|x| x.vertex - 1)),
            [
                parsed[0].normal? - 1,
                parsed[1].normal? - 1,
                parsed[2].normal? - 1,
            ],
        ))
    }

    fn parse_vector(line: Vec<&str>) -> Vector3<f64> {
        let mut words = line.into_iter();
        words.next().unwrap();
        Vector3::new(
            words.next().unwrap().parse::<f64>().unwrap(),
            words.next().unwrap().parse::<f64>().unwrap(),
            words.next().unwrap().parse::<f64>().unwrap(),
        ) * PIXELS_PER_METER
    }
}

#[derive(Default)]
pub struct ObjWriter {
    vertices: Vec<String>,
    normals: Vec<String>,
    textures: Vec<String>,
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
        for t in self.textures.iter() {
            text.push_str(&format!("vt {}", t));
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
        file.write_all(text.as_bytes())?;
        Ok(())
    }

    pub fn add_vertex(&mut self, v: &Vector3<f64>) {
        self.vertices.push(self.write_vector(v));
    }
    pub fn add_texture(&mut self, t: &Vector3<f64>) {
        self.textures.push(self.write_vector(t));
    }
    pub fn add_normal(&mut self, n: &Vector3<f64>) {
        self.normals.push(self.write_vector(n));
    }
    pub fn add_triangle(&mut self, t: &Triangle) {
        self.faces.push(self.write_triangle(t));
    }
    pub fn add_ray(&mut self, r: &Ray, t: f64) {
        let num_vertices = self.num_vertices();
        self.add_vertex(r.orig());
        self.add_vertex(&(r.orig() + r.dir() * t));
        let text = format!("{} {}", num_vertices, num_vertices + 1);
        self.lines.push(text);
    }

    fn write_triangle(&self, t: &Triangle) -> String {
        let mut text = String::new();
        for i in 0..3 {
            text.push_str(&format!("{}//{} ", t.vertices[i], t.normals[i]));
        }
        println!("{}", text);
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

    pub fn add_mesh(&mut self, m: &Mesh) {
        for v in m.vertices.iter() {
            self.add_vertex(v);
        }
        for n in m.normals.iter() {
            self.add_normal(n);
        }
        for t in m.triangles.iter() {
            self.add_triangle(t);
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }
    pub fn num_normals(&self) -> usize {
        self.normals.len()
    }
    pub fn num_textures(&self) -> usize {
        self.textures.len()
    }
}
