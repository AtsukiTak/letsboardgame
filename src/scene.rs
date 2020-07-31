use crate::{core::color::Color, meshes::Mesh};

pub struct Scene {
    meshes: Vec<Mesh>,
    pub background: Color,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            meshes: Vec::new(),
            background: Color::black(),
        }
    }

    pub fn meshes(&self) -> &[Mesh] {
        &self.meshes[..]
    }

    pub fn add(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }
}
