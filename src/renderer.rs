use crate::image::Image;
use crate::mesh::{obj::ObjWriter, Mesh};
use crate::ray::Ray;
use crate::vector::Vector3;

#[derive(Debug)]
pub struct Renderer {
    objects: Vec<Mesh>,
    camera_pos: Vector3<f64>,
    viewport_ul: Vector3<f64>,
    viewport_size: (usize, usize), // w, h
}

impl Renderer {
    pub fn new(
        camera_pos: Vector3<f64>,
        viewport_ul: Vector3<f64>,
        viewport_w: usize,
        aspect_ratio: (usize, usize),
    ) -> Renderer {
        Renderer {
            objects: Vec::new(),
            camera_pos,
            viewport_ul,
            viewport_size: (viewport_w, viewport_w * aspect_ratio.1 / aspect_ratio.0),
        }
    }

    pub fn add_mesh_from_file(&mut self, path: &str) -> Result<(), std::io::Error> {
        self.objects.push(Mesh::new_from_file(path)?);
        Ok(())
    }

    fn color_ray(&self, r: &Ray) -> Vector3<u8> {
        for o in self.objects.iter() {
            if o.hits(r).is_some() {
                println!("yay");
                return Vector3::new(255, 255, 255);
            }
        }
        Vector3::<u8>::default()
    }

    pub fn render(&self) -> Image {
        let mut writer = ObjWriter::new();
        let mut image = Image::new(self.viewport_size);
        for i in 0..self.viewport_size.0 {
            for j in 0..self.viewport_size.1 {
                let pixel_pos = &self.viewport_ul + Vector3::new(i as f64, 0.0, -(j as f64));
                let dir = &pixel_pos - &self.camera_pos;
                let r = Ray::new(pixel_pos, dir);
                println!("{:?}", r);
                image[(i, j)] = self.color_ray(&r);
                writer.add_ray(&r, 2.0);
            }
        }
        writer.add_mesh(&self.objects[0]);
        writer.write("debug.obj").unwrap();
        image
    }
}
