use cgmath::{prelude::*, BaseFloat, Matrix4};
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepVec<A>
where
    A: Array,
{
    vec: Vec<A::Element>,
    _arr: PhantomData<A>,
}

impl<A> StepVec<A>
where
    A: Array,
{
    pub fn new() -> Self {
        StepVec {
            vec: Vec::new(),
            _arr: PhantomData,
        }
    }

    pub fn step() -> usize {
        A::len()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn push(&mut self, array: A) {
        for i in 0..A::len() {
            self.vec.push(array[i]);
        }
    }
}

impl<A> AsRef<[A::Element]> for StepVec<A>
where
    A: Array,
{
    fn as_ref(&self) -> &[A::Element] {
        self.vec.as_ref()
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
