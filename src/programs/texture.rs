use super::BasicParams;
use crate::core::{
    program::{Attribute, ParamsBase, ParamsVisitor, Program, Uniform},
    shader::{FragmentShader, VertexShader},
    types::Vec2,
};
use wasm_bindgen::prelude::*;

pub struct TextureProgram {
    program: Program<TextureParams>,
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

        let program = Program::<TextureParams>::new(vert_shader, frag_shader)?;

        Ok(TextureProgram { program })
    }

    pub(crate) fn params(&self) -> &TextureParams {
        &self.program.params
    }

    pub(crate) fn params_mut(&mut self) -> &mut TextureParams {
        &mut self.program.params
    }

    pub(crate) fn use_program(&self) {
        self.program.use_program()
    }
}

pub struct TextureParams {
    pub basic: BasicParams,
    pub tex_coord: Attribute<Vec2<f32>>,
    pub texture: Uniform<i32>,
}

impl ParamsBase for TextureParams {
    fn from_visitor<'a>(visitor: ParamsVisitor<'a>) -> Result<Self, JsValue> {
        Ok(TextureParams {
            basic: BasicParams::from_visitor(visitor)?,
            tex_coord: visitor.visit_attr("texCoord")?,
            texture: visitor.visit_uniform("uTexture")?,
        })
    }
}

impl AsRef<BasicParams> for TextureParams {
    fn as_ref(&self) -> &BasicParams {
        &self.basic
    }
}

impl AsMut<BasicParams> for TextureParams {
    fn as_mut(&mut self) -> &mut BasicParams {
        &mut self.basic
    }
}
