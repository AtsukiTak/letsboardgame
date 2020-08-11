mod rect;
mod sphere;
mod torus;

pub use rect::{rect, rect_with_texture};
pub use sphere::sphere;
pub use torus::torus;

use crate::core::{
    buffers::{IBO, VBO},
    texture::Texture,
    types::{Vec2, Vec3, Vec4},
};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub positions_vbo: Rc<VBO<Vec3<f32>>>,
    pub colors_vbo: Rc<VBO<Vec4<f32>>>,
    pub normals_vbo: Rc<VBO<Vec3<f32>>>,
    pub indexes_ibo: Rc<IBO<Vec3<i16>>>,
    pub index_len: i32,
    pub texture: Option<MeshTexture>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeshTexture {
    pub coord: Rc<VBO<Vec2<f32>>>,
    pub data: Rc<Texture>,
}

impl Mesh {
    pub fn new(
        positions: Vec3<f32>,
        colors: Vec4<f32>,
        normals: Vec3<f32>,
        indexes: Vec3<i16>,
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

    pub fn paste_texture(&mut self, coord: Vec2<f32>, data: Texture) {
        self.texture = Some(MeshTexture {
            coord: Rc::new(VBO::with_data(&coord)),
            data: Rc::new(data),
        });
    }
}
