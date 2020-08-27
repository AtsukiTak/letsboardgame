mod rect;
mod sphere;
mod torus;

pub use rect::{rect, rect_with_texture};
pub use sphere::sphere;
pub use torus::torus;

use crate::texture::Texture;
use cgmath::{Vector2, Vector3, Vector4};
use napier_webgl::{
    buffers::{IBO, VBO},
    vec::StepVec,
};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub positions_vbo: Rc<VBO<StepVec<Vector3<f32>>>>,
    pub colors_vbo: Rc<VBO<StepVec<Vector4<f32>>>>,
    pub normals_vbo: Rc<VBO<StepVec<Vector3<f32>>>>,
    pub indexes_ibo: Rc<IBO<StepVec<Vector3<i16>>>>,
    pub index_len: i32,
    pub texture: Option<MeshTexture>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeshTexture {
    pub coord: Rc<VBO<StepVec<Vector2<f32>>>>,
    pub data: Rc<Texture>,
}

impl Mesh {
    pub fn new(
        positions: StepVec<Vector3<f32>>,
        colors: StepVec<Vector4<f32>>,
        normals: StepVec<Vector3<f32>>,
        indexes: StepVec<Vector3<i16>>,
    ) -> Mesh {
        Mesh {
            positions_vbo: Rc::new(VBO::with_data(&positions)),
            colors_vbo: Rc::new(VBO::with_data(&colors)),
            normals_vbo: Rc::new(VBO::with_data(&normals)),
            indexes_ibo: Rc::new(IBO::with_data(&indexes)),
            index_len: indexes.as_ref().len() as i32,
            texture: None,
        }
    }

    pub fn paste_texture(&mut self, coord: StepVec<Vector2<f32>>, data: Texture) {
        self.texture = Some(MeshTexture {
            coord: Rc::new(VBO::with_data(&coord)),
            data: Rc::new(data),
        });
    }
}
