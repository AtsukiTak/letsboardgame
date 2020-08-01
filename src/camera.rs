use crate::cell::{Cell, Vector3Cell};
use cgmath::Rad;
use derive_more::Deref;
use std::rc::Rc;

#[derive(Debug, Clone, Deref)]
#[deref(forward)]
pub struct Camera(Rc<CameraCell>);

#[derive(Debug)]
pub struct CameraCell {
    /// カメラの位置
    pub pos: Vector3Cell<f32>,
    /// カメラの方向
    pub look_for: Vector3Cell<f32>,
    /// カメラの上方向
    pub up: Vector3Cell<f32>,

    /// 画角
    pub fovy: Cell<Rad<f32>>,
    /// アスペクト比
    pub aspect: Cell<f32>,
    /// どれくらい近くまで写すか
    pub near: Cell<f32>,
    /// どれくらい遠くまで写すか
    pub far: Cell<f32>,
}
