use std::{
    fs::File,
    io::Write,
    ops::{Index, IndexMut},
};

use crate::vector::Vector3;

#[derive(Debug)]
pub struct Image {
    pixels: Vec<Vector3<u8>>,
    shape: (usize, usize),
}

impl Image {
    pub fn new(shape: (usize, usize)) -> Image {
        Image {
            pixels: vec![Vector3::<u8>::default(); shape.0 * shape.1],
            shape,
        }
    }

    pub fn write_to_ppm(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        let mut text = String::new();
        text.push_str(&format!("P6\n{} {}\n255\n", self.shape.0, self.shape.1));
        for (i, p) in self.pixels.iter().enumerate() {
            text.push_str(&format!("{} {} {}", p[0], p[1], p[2]));
            if i == self.shape.0 - 1 {
                text.push('\n');
            } else {
                text.push(' ');
            }
        }
        file.write_all(text.as_bytes()).unwrap();
    }
}

impl Index<(usize, usize)> for Image {
    type Output = Vector3<u8>;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.pixels[index.0 + index.1 * self.shape.0]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[index.0 + index.1 * self.shape.0]
    }
}
