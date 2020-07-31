use crate::core::{
    buffers::{IBO, VBO},
    context::{self, Context},
    program::{Attribute, ParamsBase, ParamsVisitor, Program, Uniform},
    shader::{FragmentShader, VertexShader},
    types::{Mat4, Vec3, Vec4},
};
use crate::models::Model;
use cgmath::{Vector3, Vector4};
use wasm_bindgen::prelude::*;

pub struct StdProgram {
    pub program: Program<Params>,
    pub model: Model,
}

impl StdProgram {
    pub fn new(model: Model) -> Result<Self, JsValue> {
        let vert_shader = VertexShader::compile(include_str!("standard.vert"))?;
        let frag_shader = FragmentShader::compile(include_str!("standard.frag"))?;

        let program = Program::<Params>::new(vert_shader, frag_shader)?;

        // "position" attributeの設定
        let vert_vbo = VBO::with_data(&model.positions);
        program.params.position.attach_vbo(&vert_vbo);

        // "normal" attributeの設定
        let normal_vbo = VBO::with_data(&model.normals);
        program.params.normal.attach_vbo(&normal_vbo);

        // "color" attributeの設定
        let colors_vbo = VBO::with_data(&model.colors);
        program.params.color.attach_vbo(&colors_vbo);

        // Index Bufferの設定
        let ibo = IBO::with_data(&model.indexes);
        ibo.bind();

        Ok(StdProgram { program, model })
    }

    pub fn params_mut(&mut self) -> &mut Params {
        &mut self.program.params
    }

    pub fn render(&self) {
        context::with(|ctx| {
            ctx.draw_elements_with_i32(
                Context::TRIANGLES,
                self.model.indexes.as_ref().len() as i32,
                Context::UNSIGNED_SHORT,
                0,
            );
        })
    }
}

pub struct Params {
    pub position: Attribute<Vec3<f32>>,
    pub normal: Attribute<Vec3<f32>>,
    pub color: Attribute<Vec4<f32>>,
    pub mvp_matrix: Uniform<Mat4<f32>>,
    pub inv_matrix: Uniform<Mat4<f32>>,
    pub light_direction: Uniform<Vector3<f32>>,
    pub eye_direction: Uniform<Vector3<f32>>,
    pub ambient_color: Uniform<Vector4<f32>>,
}

impl ParamsBase for Params {
    fn from_visitor<'a>(visitor: ParamsVisitor<'a>) -> Result<Self, JsValue> {
        Ok(Params {
            position: visitor.visit_attr("position")?,
            normal: visitor.visit_attr("normal")?,
            color: visitor.visit_attr("color")?,
            mvp_matrix: visitor.visit_uniform("mvpMatrix")?,
            inv_matrix: visitor.visit_uniform("invMatrix")?,
            light_direction: visitor.visit_uniform("lightDirection")?,
            eye_direction: visitor.visit_uniform("eyeDirection")?,
            ambient_color: visitor.visit_uniform("ambientColor")?,
        })
    }
}
