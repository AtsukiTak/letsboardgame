mod torus;

pub use torus::torus;

use crate::core::types::{Vec3, Vec4};

#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    pub positions: Vec3<f32>,
    pub colors: Vec4<f32>,
    pub indexes: Vec3<i16>,
}
