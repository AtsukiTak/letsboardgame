mod sphere;
mod torus;

pub use sphere::sphere;
pub use torus::torus;

use crate::core::types::{Vec3, Vec4};

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub positions: Vec3<f32>,
    pub colors: Vec4<f32>,
    pub indexes: Vec3<i16>,
    pub normals: Vec3<f32>,
}
