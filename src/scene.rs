use crate::{Color, Light, Object};

pub struct Scene {
    objects: Vec<Object>,
    pub background: Color,
    pub ambient_color: Color,
    pub light: Option<Light>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            background: Color::black(),
            ambient_color: Color::rgba(25, 25, 25, 0.1),
            light: None,
        }
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects[..]
    }

    pub fn add(&mut self, object: &Object) {
        self.objects.push(object.shared_clone());
    }

    pub fn remove(&mut self, _object: &Object) {
        unimplemented!();
    }
}
