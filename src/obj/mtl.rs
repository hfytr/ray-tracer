use std::{collections::HashMap, fs::read_to_string};

use super::parse::parse_vector;
use crate::scene::mesh::Material;

#[derive(Default, Clone)]
pub struct MtlParser {
    pub materials: Vec<Material>,
    pub material_indices: HashMap<String, usize>,
    cur_material: (String, Material),
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
                self.materials.push(self.cur_material.1.clone());
                self.cur_material = (line[1].to_string(), Material::default());
            }
            "Ke" => {
                self.cur_material.1.emission = parse_vector(line);
            }
            "Kd" => {
                self.cur_material.1.diffuse = parse_vector(line);
            }
            // "Ka" => {
            //     self.cur_material.1.ambient = Self::parse_vector(line);
            // }
            // "Ks" => {
            //     self.cur_material.1.specular = Self::parse_vector(line);
            // }
            // "Ns" => {
            //     self.cur_material.1.specular_exp = line[1].parse::<f64>().unwrap();
            // }
            // "Ni" => {
            //     self.cur_material.1.density = line[1].parse::<f64>().unwrap();
            // }
            // "d" => {
            //     self.cur_material.1.opacity = line[1].parse::<f64>().unwrap();
            // }
            // "Tr" => {
            //     self.cur_material.1.opacity = 1.0 - line[1].parse::<f64>().unwrap();
            // }
            _ => {}
        }
    }
}
