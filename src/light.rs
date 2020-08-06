use cgmath::{vec3, Vector3};

pub enum Light {
    Directional(DirectionalLight),
}

impl Light {
    pub fn directional(x: f32, y: f32, z: f32) -> Light {
        Light::Directional(DirectionalLight::new(x, y, z))
    }
}

/// 平行光源
pub struct DirectionalLight {
    pub dir: Vector3<f32>,
}

impl DirectionalLight {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        DirectionalLight { dir: vec3(x, y, z) }
    }
}
