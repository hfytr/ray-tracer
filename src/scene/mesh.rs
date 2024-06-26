use crate::vector::Vector3;

#[derive(Debug, Default, Clone)]
pub struct Triangle {
    pub vertices: [usize; 3],
    pub point_normals: [usize; 3],
    pub material: usize,
    pub face_normal: usize,
}

#[derive(Debug, Default, Clone)]
pub struct Material {
    pub diffuse: Vector3<f64>,
    pub emission: Vector3<f64>,
    // pub ambient: Vector3<f64>,
    // pub specular: Vector3<f64>,
    // pub filter: Vector3<f64>,
    // pub specular_exp: f64,
    // pub opacity: f64,
    // pub density: f64,
}
