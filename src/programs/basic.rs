use crate::core::{
    program::{Attribute, ParamsBase, ParamsVisitor, Program, Uniform},
    shader::{FragmentShader, VertexShader},
    types::{Mat4, StepVec},
};
use cgmath::{Vector3, Vector4};
use wasm_bindgen::prelude::*;

pub struct BasicProgram {
    program: Program<BasicParams>,
}

impl BasicProgram {
    /// フォンシェーディング版のBasicProgramを生成する
    pub fn phong() -> Result<Self, JsValue> {
        let vert_shader = VertexShader::compile(include_str!("phong.vert"))?;
        let frag_shader = FragmentShader::compile(include_str!("phong.frag"))?;

        let program = Program::<BasicParams>::new(vert_shader, frag_shader)?;

        Ok(BasicProgram { program })
    }

    /// グーローシェーディング版のBasicProgramを生成する
    pub fn gouraud() -> Result<Self, JsValue> {
        let vert_shader = VertexShader::compile(include_str!("gouraud.vert"))?;
        let frag_shader = FragmentShader::compile(include_str!("gouraud.frag"))?;

        let program = Program::<BasicParams>::new(vert_shader, frag_shader)?;

        Ok(BasicProgram { program })
    }

    pub(crate) fn params(&self) -> &BasicParams {
        &self.program.params
    }

    pub(crate) fn params_mut(&mut self) -> &mut BasicParams {
        &mut self.program.params
    }

    pub(crate) fn use_program(&self) {
        self.program.use_program()
    }
}

pub struct BasicParams {
    // for vertex shader
    pub position: Attribute<StepVec<Vector3<f32>>>,
    pub normal: Attribute<StepVec<Vector3<f32>>>,
    pub color: Attribute<StepVec<Vector4<f32>>>,
    pub mvp_matrix: Uniform<Mat4<f32>>,
    pub m_matrix: Uniform<Mat4<f32>>,

    // for fragment shader
    pub inv_m_matrix: Uniform<Mat4<f32>>,
    pub light_type: Uniform<i32>,
    pub light_val: Uniform<Vector3<f32>>,
    pub eye_direction: Uniform<Vector3<f32>>,
    pub ambient_color: Uniform<Vector4<f32>>,
}

impl ParamsBase for BasicParams {
    fn from_visitor<'a>(visitor: ParamsVisitor<'a>) -> Result<Self, JsValue> {
        Ok(BasicParams {
            // for vertex shader
            position: visitor.visit_attr("position")?,
            normal: visitor.visit_attr("normal")?,
            color: visitor.visit_attr("color")?,
            mvp_matrix: visitor.visit_uniform("mvpMatrix")?,
            m_matrix: visitor.visit_uniform("mMatrix")?,

            // for fragment shader
            inv_m_matrix: visitor.visit_uniform("invMMatrix")?,
            light_type: visitor.visit_uniform("lightType")?,
            light_val: visitor.visit_uniform("lightVal")?,
            eye_direction: visitor.visit_uniform("eyeDirection")?,
            ambient_color: visitor.visit_uniform("ambientColor")?,
        })
    }
}
