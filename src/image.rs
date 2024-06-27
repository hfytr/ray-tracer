use std::{
    fs::File,
    io::BufWriter,
    ops::{Index, IndexMut},
};

use crate::vector::Vector3;

#[derive(Debug)]
pub struct Image {
    pixels: Vec<Vector3<f64>>,
    shape: (usize, usize),
}

impl Image {
    pub fn new(shape: (usize, usize)) -> Image {
        Image {
            pixels: vec![Vector3::<f64>::default(); shape.0 * shape.1],
            shape,
        }
    }

    pub fn write_to_png(&self, path: &str) -> Result<(), std::io::Error> {
        let file = File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.shape.0 as u32, self.shape.1 as u32);

        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
        let source_chromaticities = png::SourceChromaticities::new(
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header()?;

        writer.write_image_data(&self.as_flattened_u8())?;

        Ok(())
    }

    fn as_flattened_u8(&self) -> Vec<u8> {
        self.pixels
            .iter()
            .flat_map(Vector3::as_vec)
            .map(|x| x as u8)
            .collect()
    }
}

impl Index<(usize, usize)> for Image {
    type Output = Vector3<f64>;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.pixels[index.0 + index.1 * self.shape.0]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[index.0 + index.1 * self.shape.0]
    }
}
