use crate::meshes::Mesh;
use cgmath::{vec3, Matrix4, Rad, Vector3};
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
/// transform.rotate.angle += Rad(1.0);
/// transform.pos.x += 1;
#[derive(Debug, Clone)]
pub struct Transform {
    // 移動方向
    pub pos: AtomicVector3,
    // 回転軸
    pub rotate: TransformRotate,
    // x, y, z 方向への拡大率
    pub scale: AtomicVector3,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            pos: AtomicVector3::zero(),
            rotate: TransformRotate::new(),
            scale: AtomicVector3::new(1.0, 1.0, 1.0),
        }
    }

    // モデル座標変換行列を計算する
    pub fn matrix(&self) -> Matrix4<f32> {
        let move_matrix = Matrix4::from_translation(self.pos.get());
        let rotate_matrix =
            Matrix4::from_axis_angle(self.rotate.axis.get(), self.rotate.angle.get());
        let scale = self.scale.get();
        let scale_matrix = Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);

        // 本来は scale, rotate, move の順で適用するが、
        // WebGLは列オーダーなため、順序が逆になり
        // move, rotate, scale の順でかけていく。
        move_matrix * rotate_matrix * scale_matrix
    }
}

#[derive(Debug, Clone)]
pub struct AtomicVector3 {
    pub x: Atomic<f32>,
    pub y: Atomic<f32>,
    pub z: Atomic<f32>,
}

impl AtomicVector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        AtomicVector3 {
            x: Atomic::new(x),
            y: Atomic::new(y),
            z: Atomic::new(z),
        }
    }

    pub fn zero() -> Self {
        AtomicVector3::new(0.0, 0.0, 0.0)
    }

    pub fn get(&self) -> Vector3<f32> {
        vec3(self.x.get(), self.y.get(), self.z.get())
    }
}

#[derive(Debug, Clone)]
pub struct Atomic<T: Copy>(Cell<T>);

impl<T> Atomic<T>
where
    T: Copy,
{
    pub fn new(t: T) -> Self {
        Atomic(Cell::new(t))
    }

    pub fn get(&self) -> T {
        self.0.get()
    }

    pub fn set(&self, t: T) {
        self.0.set(t);
    }
}

impl<T, U> AddAssign<U> for &Atomic<T>
where
    T: AddAssign<U> + Copy,
{
    fn add_assign(&mut self, other: U) {
        let mut t = self.get();
        t += other;
        self.set(t);
    }
}

#[derive(Debug, Clone)]
pub struct TransformRotate {
    pub axis: AtomicVector3,
    pub angle: Atomic<Rad<f32>>,
}

impl TransformRotate {
    fn new() -> Self {
        TransformRotate {
            axis: AtomicVector3::new(1.0, 0.0, 0.0),
            angle: Atomic::new(Rad(0.0)),
        }
    }
}
