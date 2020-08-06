use crate::{
    cell::{Cell, Vector3Cell},
    meshes::Mesh,
};
use cgmath::{prelude::*, Matrix4, Rad};
use std::rc::Rc;

/// world 上のオブジェクト
/// `Mesh` を拡大縮小、回転、移動させたもの
#[derive(Debug)]
pub struct Object {
    pub mesh: Mesh,
    pub transform: Rc<Transform>,
}

impl Object {
    /// 新しいObjectを生成する。
    pub fn new(mesh: Mesh) -> Self {
        Object {
            mesh,
            transform: Rc::new(Transform::new()),
        }
    }

    /// 同じObjectを指すObjectを生成する。
    /// 通常のCloneと違い、生成されたObjectは実態としては
    /// 生成元と同じなため、一方に変更を加えると
    /// もう一方も変更される。
    pub fn shared_clone(&self) -> Object {
        Object {
            mesh: self.mesh.clone(),
            transform: self.transform.clone(),
        }
    }
}

/// 新しいObjectを生成する。
/// 生成されたObjectは生成元と完全に異なるObjectとなるため、
/// 一方に変更を加えても、もう一方に影響を与えない。
impl Clone for Object {
    fn clone(&self) -> Object {
        Object {
            mesh: self.mesh.clone(),
            transform: Rc::new(Transform::clone(&self.transform)),
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
#[derive(Debug, Clone)]
pub struct Transform {
    // 移動方向
    pub pos: Vector3Cell<f32>,
    // 回転軸
    pub rotate: TransformRotate,
    // x, y, z 方向への拡大率
    pub scale: Vector3Cell<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            pos: Vector3Cell::zero(),
            rotate: TransformRotate::new(),
            scale: Vector3Cell::new(1.0, 1.0, 1.0),
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

#[derive(Debug, Clone)]
pub struct TransformRotate {
    pub axis: Vector3Cell<f32>,
    pub angle: Cell<Rad<f32>>,
}

impl TransformRotate {
    fn new() -> Self {
        TransformRotate {
            axis: Vector3Cell::new(1.0, 0.0, 0.0),
            angle: Cell::new(Rad(0.0)),
        }
    }
}
