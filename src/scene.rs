use crate::{core::color::Color, object::Object};

pub struct Scene {
    objects: Vec<Object>,
    pub background: Color,
    pub ambient_color: Color,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            background: Color::black(),
            ambient_color: Color::rgba(25, 25, 25, 0.1),
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
