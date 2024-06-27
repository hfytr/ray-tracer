use std::{collections::HashMap, fs::read_to_string};

use super::parse::parse_vector;
use crate::scene::mesh::Material;

#[derive(Default, Debug, Clone)]
pub struct MtlParser {
    pub materials: Vec<Material>,
    pub material_indices: HashMap<String, usize>,
}

impl MtlParser {
    pub fn parse(&mut self, path: &str) -> Result<Self, std::io::Error> {
        let file_string = read_to_string(path)?;
        let lines: Vec<Vec<&str>> = file_string
            .lines()
            .filter(|s| s.chars().next().unwrap_or('#') != '#')
            .map(|s| s.split(' ').collect())
            .collect();
        for line in lines.iter() {
            self.parse_line(line)
        }
        Ok(self.clone())
    }
    fn parse_line(&mut self, line: &[&str]) {
        match line[0] {
            "newmtl" => {
                self.material_indices
                    .insert(line[1].to_string(), self.materials.len());
                self.materials.push(Material::default());
            }
            "Ke" => {
                self.materials.last_mut().unwrap().emission = parse_vector(line);
            }
            "Kd" => {
                self.materials.last_mut().unwrap().diffuse = parse_vector(line);
            }
            // "Ka" => {
            //     self.materials.last_mut().unwrap().ambient = Self::parse_vector(line);
            // }
            // "Ks" => {
            //     self.materials.last_mut().unwrap().specular = Self::parse_vector(line);
            // }
            // "Ns" => {
            //     self.materials.last_mut().unwrap().specular_exp = line[1].parse::<f64>().unwrap();
            // }
            // "Ni" => {
            //     self.materials.last_mut().unwrap().density = line[1].parse::<f64>().unwrap();
            // }
            // "d" => {
            //     self.materials.last_mut().unwrap().opacity = line[1].parse::<f64>().unwrap();
            // }
            // "Tr" => {
            //     self.materials.last_mut().unwrap().opacity = 1.0 - line[1].parse::<f64>().unwrap();
            // }
            _ => {}
        }
    }
}
