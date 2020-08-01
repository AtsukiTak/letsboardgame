use cgmath::{prelude::*, Deg, Matrix4, Point3, Rad, Vector3};
use derive_more::Deref;
use std::rc::Rc;

/// カメラを表すモデル
///
/// ```rust
/// let camera = Camera::new();
/// camera.pos.set(0.0, 0.0, 20.0);
/// ```
#[derive(Debug, Clone, Deref)]
#[deref(forward)]
pub struct Camera(Rc<CameraCell>);

#[derive(Debug)]
pub struct CameraCell {
    /// カメラの位置
    /// Default : (0.0, 0.0, 0.0),
    pub pos: Vector3<f32>,
    /// カメラの注視点
    /// Default : (0.0, 0.0, 0.0),
    pub look_at: Vector3<f32>,
    /// カメラの上方向
    /// Default : (0.0, 0.0, 1.0),
    pub up: Vector3<f32>,

    /// 画角
    /// Default : Deg(45.0)
    pub fovy: Rad<f32>,
    /// アスペクト比
    /// Default : 1.0
    pub aspect: f32,
    /// どれくらい近くまで写すか
    /// Default : 0.1
    pub near: f32,

    /// どれくらい遠くまで写すか
    /// Default : 100.0
    pub far: f32,
}

impl Camera {
    pub fn new() -> Self {
        let inner = CameraCell {
            pos: Vector3::zero(),
            look_at: Vector3::zero(),
            up: Vector3::new(0.0, 0.0, 1.0),
            fovy: Deg(45.0).into(),
            aspect: 1.0,
            near: 0.1,
            far: 100.0,
        };
        Camera(Rc::new(inner))
    }

    /// ( プロジェクション座標変換行列 ) * ( ビュー座標変換行列 ) の計算結果を返す
    pub(crate) fn matrix(&self) -> Matrix4<f32> {
        let p_mat = cgmath::perspective(self.fovy, self.aspect, self.near, self.far);

        let v_mat = Matrix4::look_at(
            Point3::from_vec(self.pos),
            Point3::from_vec(self.look_at),
            self.up,
        );

        p_mat * v_mat
    }
}
