use cgmath::{prelude::*, BaseFloat, Matrix4};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vec3<T>(pub Vec<T>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vec4<T>(pub Vec<T>);

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

pub struct Mat4<T>(Matrix4<T>);

impl<T> Mat4<T>
where
    T: BaseFloat,
{
    pub fn new(mat: Matrix4<T>) -> Self {
        Mat4(mat)
    }
}

impl<T> AsRef<[T]> for Mat4<T>
where
    T: BaseFloat,
{
    fn as_ref(&self) -> &[T] {
        AsRef::<[T; 16]>::as_ref(&self.0).as_ref()
    }
}

impl<T> Default for Mat4<T>
where
    T: BaseFloat,
{
    fn default() -> Self {
        Mat4(Matrix4::identity())
    }
}

impl<T> From<Matrix4<T>> for Mat4<T>
where
    T: BaseFloat,
{
    fn from(value: Matrix4<T>) -> Self {
        Mat4(value)
    }
}
