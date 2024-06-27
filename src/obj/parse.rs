use std::{collections::HashMap, fs::read_to_string, str::FromStr};

use super::mtl::MtlParser;
use crate::{
    scene::mesh::{Material, Triangle},
    vector::Vector3,
};

const PIXELS_PER_METER: f64 = 100.0;

pub fn parse_vector(line: &[&str]) -> Vector3<f64> {
    let mut words = line.iter();
    words.next().unwrap();
    Vector3::new(
        words.next().unwrap().parse::<f64>().unwrap(),
        words.next().unwrap().parse::<f64>().unwrap(),
        words.next().unwrap().parse::<f64>().unwrap(),
    )
}

fn collect_array<T, const N: usize>(mut i: impl Iterator<Item = T>) -> [T; N] {
    std::array::from_fn(|_| i.next().unwrap())
}

pub struct FaceEntryElement {
    pub vertex: usize,
    pub texture: Option<usize>,
    pub normal: Option<usize>,
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

#[derive(Default, Debug)]
pub struct ObjParser {
    pub point_normals: Vec<Vector3<f64>>,
    pub face_normals: Vec<Vector3<f64>>,
    pub vertices: Vec<Vector3<f64>>,
    pub materials: Vec<Material>,
    pub triangles: Vec<Triangle>,
    pub meshes: Vec<usize>,
    material_indices: HashMap<String, usize>,
    cur_material: usize,
}

impl ObjParser {
    pub fn parse(&mut self, path: &str) -> Result<(), std::io::Error> {
        let file_string = read_to_string(path)?;
        let lines: Vec<Vec<&str>> = file_string
            .lines()
            .filter(|s| s.chars().next().unwrap_or('#') != '#')
            .map(|s| s.split(' ').collect())
            .collect();
        for mut line in lines {
            self.parse_line(&mut line, path)?;
        }
        Ok(())
    }

    fn parse_line(&mut self, line: &mut Vec<&str>, path: &str) -> Result<(), std::io::Error> {
        match line[0] {
            "mtllib" => {
                let mut mtl_name: String = path.to_string();
                // get folder
                while !mtl_name.is_empty() && mtl_name.pop().unwrap() != '/' {}
                if !mtl_name.is_empty() {
                    mtl_name.push('/');
                }
                mtl_name.push_str(line[1]);

                let MtlParser {
                    materials,
                    material_indices,
                    ..
                } = MtlParser::default().parse(&mtl_name)?;
                self.materials = materials;
                self.material_indices = material_indices;
            }
            "o" => self.meshes.push(self.triangles.len()),
            "f" => {
                assert_eq!(
                    4,
                    line.len(),
                    "this face is not a triangle, please triangulate the surface before parsing"
                );
                let triangle = self.parse_face(line).expect("normals not provided");
                self.triangles.push(triangle);
            }
            "vn" => {
                let mut normal = parse_vector(line) * PIXELS_PER_METER;
                normal.normalize();
                self.point_normals.push(normal);
            }
            "v" => {
                self.vertices.push(parse_vector(line) * PIXELS_PER_METER);
            }
            "usemtl" => {
                self.cur_material = *self
                    .material_indices
                    .get(line[1])
                    .expect("invalid material");
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_face(&mut self, line: &[&str]) -> Option<Triangle> {
        assert_eq!(
            line.len(),
            4,
            "please triangulate the obj file, then try again"
        );
        let parsed: Vec<FaceEntryElement> = line
            .iter()
            .skip(1)
            .map(|s| FaceEntryElement::new(s).expect("invalid face element"))
            .collect();
        for p in parsed.iter() {
            if p.texture.is_some() {
                println!("texture not currently supported");
                break;
            }
        }
        let triangle = Triangle {
            vertices: collect_array(parsed.iter().map(|x| x.vertex - 1)),
            point_normals: [
                parsed[0].normal? - 1,
                parsed[1].normal? - 1,
                parsed[2].normal? - 1,
            ],
            face_normal: self.face_normals.len(),
            material: self.cur_material,
        };
        self.face_normals
            .push(self.calculate_face_normal(&triangle, true));
        Some(triangle)
    }

    fn calculate_face_normal(&self, triangle: &Triangle, clockwise: bool) -> Vector3<f64> {
        let v0 = &self.vertices[triangle.vertices[0]];
        let v1 = &self.vertices[triangle.vertices[1]];
        let v2 = &self.vertices[triangle.vertices[2]];

        let e0 = v1 - v0;
        let e1 = v2 - v0;

        let mut normal = e1.cross(&e0);
        normal.normalize();
        if !clockwise {
            normal *= -1.0;
        }

        normal
    }
}
