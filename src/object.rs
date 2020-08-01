use crate::meshes::Mesh;
use cgmath::{prelude::*, Matrix4, Rad, Vector3};
use std::{cell::Cell, rc::Rc};

/// world 上のオブジェクト
/// `Mesh` を拡大縮小、回転、移動させたもの
#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub(crate) mesh: Mesh,
    pub(crate) transform: Rc<Cell<Transform>>,
}

impl Object {
    pub fn new(mesh: Mesh) -> Self {
        Object {
            mesh,
            transform: Rc::new(Cell::new(Transform::new())),
        }
    }

    pub fn transform(&self) -> Transform {
        self.transform.get()
    }

    pub fn set_transform(&self, transform: Transform) {
        self.transform.set(transform);
    }
}

/// `Mesh` の拡大縮小、回転、移動を表現するモデル
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    // 移動方向
    pub mov: Vector3<f32>,
    // 回転軸
    pub rotate_axis: Vector3<f32>,
    // 回転量
    pub rotate_angle: Rad<f32>,
    // x, y, z 方向への拡大率
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            mov: Vector3::zero(),
            rotate_axis: Vector3::unit_x(),
            rotate_angle: Rad(0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    // モデル座標変換行列を計算する
    pub fn matrix(&self) -> Matrix4<f32> {
        let move_matrix = Matrix4::from_translation(self.mov);
        let rotate_matrix = Matrix4::from_axis_angle(self.rotate_axis, self.rotate_angle);
        let scale_matrix = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

        // 本来は scale, rotate, move の順で適用するが、
        // WebGLは列オーダーなため、順序が逆になり
        // move, rotate, scale の順でかけていく。
        move_matrix * rotate_matrix * scale_matrix
    }
}
