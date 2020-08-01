use crate::{core::color::Color, object::Object};

pub struct Scene {
    objects: Vec<Object>,
    pub background: Color,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            background: Color::black(),
        }
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects[..]
    }

    pub fn add(&mut self, object: &Object) {
        self.objects.push(object.clone());
    }
}
