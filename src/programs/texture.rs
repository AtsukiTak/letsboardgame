use crate::core::{
    program::{Attribute, ParamsBase, ParamsVisitor, Program, Uniform},
    shader::{FragmentShader, VertexShader},
    types::{Mat4, Vec2, Vec3, Vec4},
};
use cgmath::{Vector3, Vector4};
use wasm_bindgen::prelude::*;

pub struct TextureProgram {
    program: Program<Params>,
}

impl TextureProgram {
    /// フォンシェーディング版のTextureProgramを生成する
    pub fn phong() -> Result<Self, JsValue> {
        todo!()
    }

    /// グーローシェーディング版のTextureProgramを生成する
    pub fn gouraud() -> Result<Self, JsValue> {
        let vert_shader = VertexShader::compile(include_str!("texture_gouraud.vert"))?;
        let frag_shader = FragmentShader::compile(include_str!("texture_gouraud.frag"))?;

        let program = Program::<Params>::new(vert_shader, frag_shader)?;

        Ok(TextureProgram { program })
    }

    pub(crate) fn params(&self) -> &Params {
        &self.program.params
    }

    pub(crate) fn params_mut(&mut self) -> &mut Params {
        &mut self.program.params
    }

    pub(crate) fn use_program(&self) {
        self.program.use_program()
    }
}

pub struct Params {
    pub position: Attribute<Vec3<f32>>,
    pub normal: Attribute<Vec3<f32>>,
    pub color: Attribute<Vec4<f32>>,
    pub tex_coord: Attribute<Vec2<f32>>,

    pub mvp_matrix: Uniform<Mat4<f32>>,
    pub m_matrix: Uniform<Mat4<f32>>,
    pub inv_m_matrix: Uniform<Mat4<f32>>,
    pub light_type: Uniform<i32>,
    pub light_val: Uniform<Vector3<f32>>,
    pub eye_direction: Uniform<Vector3<f32>>,
    pub ambient_color: Uniform<Vector4<f32>>,
    pub texture: Uniform<i32>,
}

impl ParamsBase for Params {
    fn from_visitor<'a>(visitor: ParamsVisitor<'a>) -> Result<Self, JsValue> {
        Ok(Params {
            position: visitor.visit_attr("position")?,
            normal: visitor.visit_attr("normal")?,
            color: visitor.visit_attr("color")?,
            tex_coord: visitor.visit_attr("texCoord")?,

            mvp_matrix: visitor.visit_uniform("mvpMatrix")?,
            m_matrix: visitor.visit_uniform("mMatrix")?,
            inv_m_matrix: visitor.visit_uniform("invMMatrix")?,
            light_type: visitor.visit_uniform("lightType")?,
            light_val: visitor.visit_uniform("lightVal")?,
            eye_direction: visitor.visit_uniform("eyeDirection")?,
            ambient_color: visitor.visit_uniform("ambientColor")?,
            texture: visitor.visit_uniform("uTexture")?,
        })
    }
}
