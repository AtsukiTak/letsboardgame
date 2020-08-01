use cgmath::{vec3, Vector3};
use std::{cell::Cell, ops::AddAssign};

#[derive(Debug)]
pub struct SharedVector3<T>
where
    T: Copy,
{
    pub x: Shared<T>,
    pub y: Shared<T>,
    pub z: Shared<T>,
}

impl<T> SharedVector3<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        SharedVector3 {
            x: Shared::new(x),
            y: Shared::new(y),
            z: Shared::new(z),
        }
    }

    pub fn get(&self) -> Vector3<T> {
        vec3(self.x.get(), self.y.get(), self.z.get())
    }

    pub fn set(&self, x: T, y: T, z: T) {
        self.x.set(x);
        self.y.set(y);
        self.z.set(z);
    }
}

impl SharedVector3<f32> {
    pub fn zero() -> Self {
        SharedVector3::new(0.0, 0.0, 0.0)
    }
}

#[derive(Debug)]
pub struct Shared<T: Copy>(Cell<T>);

impl<T> Shared<T>
where
    T: Copy,
{
    pub fn new(t: T) -> Self {
        Shared(Cell::new(t))
    }

    pub fn get(&self) -> T {
        self.0.get()
    }

    pub fn set(&self, t: T) {
        self.0.set(t);
    }

    /// This method is almost equivalent to `+=` operation except that this does not require
    /// mutable reference.
    ///
    /// ```rust
    /// let v = Shared::new(1);
    /// v.add(1);
    /// assert_eq!(v.get(), 2);
    /// ```
    pub fn add<U>(&self, other: U)
    where
        T: AddAssign<U>,
    {
        let mut t = self.get();
        t += other;
        self.set(t);
    }
}
