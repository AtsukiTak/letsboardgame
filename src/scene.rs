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
}
