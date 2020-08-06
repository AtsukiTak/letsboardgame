use cgmath::{vec3, Vector3};
use std::{cell::Cell as StdCell, ops::AddAssign};

#[derive(Debug, Clone)]
pub struct Vector3Cell<T>
where
    T: Copy,
{
    pub x: Cell<T>,
    pub y: Cell<T>,
    pub z: Cell<T>,
}

impl<T> Vector3Cell<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3Cell {
            x: Cell::new(x),
            y: Cell::new(y),
            z: Cell::new(z),
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

impl Vector3Cell<f32> {
    pub fn zero() -> Self {
        Vector3Cell::new(0.0, 0.0, 0.0)
    }
}

#[derive(Debug, Clone)]
pub struct Cell<T: Copy>(StdCell<T>);

impl<T> Cell<T>
where
    T: Copy,
{
    pub fn new(t: T) -> Self {
        Cell(StdCell::new(t))
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
    /// let v = Cell::new(1);
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
