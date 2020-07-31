use crate::meshes::Mesh;

pub struct Scene {
    meshes: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Self {
        Scene { meshes: Vec::new() }
    }

    pub fn meshes(&self) -> &[Mesh] {
        &self.meshes[..]
    }

    pub fn add(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }
}
