mod torus;

pub use torus::torus;

#[derive(Debug, Clone, PartialEq)]
pub struct Vec3<T>(Vec<T>);

#[derive(Debug, Clone, PartialEq)]
pub struct Vec4<T>(Vec<T>);

impl<T> Vec3<T> {
    pub fn new() -> Self {
        Vec3(Vec::new())
    }

    pub fn push_3(&mut self, a: T, b: T, c: T) {
        self.0.push(a);
        self.0.push(b);
        self.0.push(c);
    }
}

impl<T> AsRef<[T]> for Vec3<T> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T> Vec4<T> {
    pub fn new() -> Self {
        Vec4(Vec::new())
    }

    pub fn push_4(&mut self, a: T, b: T, c: T, d: T) {
        self.0.push(a);
        self.0.push(b);
        self.0.push(c);
        self.0.push(d);
    }
}

impl<T> AsRef<[T]> for Vec4<T> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    pub positions: Vec3<f32>,
    pub colors: Vec4<f32>,
    pub indexes: Vec3<i16>,
}
