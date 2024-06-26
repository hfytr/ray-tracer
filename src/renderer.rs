use crate::image::Image;
use crate::obj::ObjWriter;
use crate::rand::NormalDist;
use crate::ray::Ray;
use crate::scene::{mesh::*, Scene};
use crate::vector::Vector3;

#[derive(Debug)]
pub struct Renderer {
    scene: Scene,
    camera_pos: Vector3<f64>,
    viewport_ul: Vector3<f64>,
    viewport_size: (usize, usize),
    rand_state: NormalDist,
    logger: ObjWriter,
}

const MAX_BOUNCES: u8 = 4;
const ACNE_THRESHOLD: f64 = 0.0001;

impl Renderer {
    pub fn new(
        camera_pos: Vector3<f64>,
        viewport_ul: Vector3<f64>,
        viewport_w: usize,
        aspect_ratio: (usize, usize),
    ) -> Renderer {
        Renderer {
            scene: Scene::default(),
            camera_pos,
            viewport_ul,
            viewport_size: (viewport_w, viewport_w * aspect_ratio.1 / aspect_ratio.0),
            rand_state: NormalDist::new(69420_u64),
            logger: ObjWriter::new(),
        }
    }

    pub fn load_obj(&mut self, path: &str) -> Result<(), std::io::Error> {
        self.scene = Scene::from_obj(path)?;
        Ok(())
    }

    fn hit_info(&self, triangle: usize) -> &Material {
        self.scene.get_triangel_mat(triangle)
    }

    fn color_ray(&mut self, init_ray: &Ray) -> Vector3<u8> {
        let mut ray_color = Vector3::<f64>::new(1.0, 1.0, 1.0);
        let mut light = Vector3::<f64>::default();
        let mut ray = Ray::default();
        for i in 0..MAX_BOUNCES {
            let cur_ray = if i == 0 { init_ray } else { &ray };
            self.logger.add_ray(cur_ray, 100.0);
            if let Some((t, triangle)) = self.scene.hits(cur_ray, ACNE_THRESHOLD) {
                let ray_dir =
                    Ray::rand_dir(&mut self.rand_state) + self.scene.get_face_normal(triangle);
                let hit_point = init_ray.point_at(t);
                ray = Ray::new(hit_point, ray_dir);

                let material = self.hit_info(triangle);
                let mut new_light = material.emission.clone();
                new_light.mul_element_wise(&ray_color);
                light += new_light;
                ray_color.mul_element_wise(&material.diffuse)
            } else {
                break;
            }
        }
        light.apply(|x| (x * 256.0) as u8)
    }

    // fn color_ray(&mut self, init_ray: &Ray, a: bool) -> Vector3<u8> {
    //     if self.scene.hits(init_ray, ACNE_THRESHOLD, a).is_some() {
    //         Vector3::<u8>::new(255, 255, 255)
    //     } else {
    //         Vector3::default()
    //     }
    // }

    pub fn render(&mut self) -> Image {
        self.logger = ObjWriter::new();
        let mut image = Image::new(self.viewport_size);
        for i in 0..self.viewport_size.0 {
            for j in 0..self.viewport_size.1 {
                let pixel_pos = &self.viewport_ul + Vector3::new(i as f64, 0.0, -(j as f64));
                let dir = &pixel_pos - &self.camera_pos;
                let r = Ray::new(pixel_pos, dir);
                if j == 0 {
                    // self.logger.add_ray(&r, 100.0);
                }
                image[(i, j)] = self.color_ray(&r);
            }
        }
        self.logger.add_scene(&self.scene, false);
        self.logger.write("debug.obj").unwrap();
        image
    }
}
