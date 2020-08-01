use crate::meshes::Mesh;
use cgmath::{prelude::*, vec3, Matrix4, Rad, Vector3};
use std::ops::AddAssign;
use std::{cell::Cell, rc::Rc};

/// world 上のオブジェクト
/// `Mesh` を拡大縮小、回転、移動させたもの
#[derive(Debug, Clone)]
pub struct Object {
    pub mesh: Mesh,
    pub transform: Rc<Transform>,
}

impl Object {
    pub fn new(mesh: Mesh) -> Self {
        Object {
            mesh,
            transform: Rc::new(Transform::new()),
        }
    }
}

/// `Mesh` の拡大縮小、回転、移動を表現するモデル
///
/// ## Example
/// let transform = Transform::new();
/// transform.rotate.axis.set(0.0, 1.0, 1.0);
/// transform.rotate.angle.add(Rad(1.0));
/// transform.pos.x.add(1);
#[derive(Debug)]
pub struct Transform {
    // 移動方向
    pub pos: SharedVector3,
    // 回転軸
    pub rotate: TransformRotate,
    // x, y, z 方向への拡大率
    pub scale: SharedVector3,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            pos: SharedVector3::zero(),
            rotate: TransformRotate::new(),
            scale: SharedVector3::new(1.0, 1.0, 1.0),
        }
    }

    // モデル座標変換行列を計算する
    pub fn matrix(&self) -> Matrix4<f32> {
        let move_matrix = Matrix4::from_translation(self.pos.get());
        let rotate_axis = self.rotate.axis.get().normalize();
        let rotate_matrix = Matrix4::from_axis_angle(rotate_axis, self.rotate.angle.get());
        let scale = self.scale.get();
        let scale_matrix = Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);

        // 本来は scale, rotate, move の順で適用するが、
        // WebGLは列オーダーなため、順序が逆になり
        // move, rotate, scale の順でかけていく。
        move_matrix * rotate_matrix * scale_matrix
    }
}

#[derive(Debug)]
pub struct SharedVector3 {
    pub x: Shared<f32>,
    pub y: Shared<f32>,
    pub z: Shared<f32>,
}

impl SharedVector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        SharedVector3 {
            x: Shared::new(x),
            y: Shared::new(y),
            z: Shared::new(z),
        }
    }

    pub fn zero() -> Self {
        SharedVector3::new(0.0, 0.0, 0.0)
    }

    pub fn get(&self) -> Vector3<f32> {
        vec3(self.x.get(), self.y.get(), self.z.get())
    }

    pub fn set(&self, x: f32, y: f32, z: f32) {
        self.x.set(x);
        self.y.set(y);
        self.z.set(z);
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

#[derive(Debug)]
pub struct TransformRotate {
    pub axis: SharedVector3,
    pub angle: Shared<Rad<f32>>,
}

impl TransformRotate {
    fn new() -> Self {
        TransformRotate {
            axis: SharedVector3::new(1.0, 0.0, 0.0),
            angle: Shared::new(Rad(0.0)),
        }
    }
}
