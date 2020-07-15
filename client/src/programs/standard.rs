use crate::core::{
    buffers::{IBO, VBO},
    context::{self, Context},
    program::Program,
    shader::{Attribute, FragmentShader, ParamsBase, ParamsVisitor, Uniform, VertexShader},
    types::{Mat4, Vec3, Vec4},
};
use crate::models::Model;
use cgmath::Matrix4;
use wasm_bindgen::prelude::*;

pub struct StdProgram {
    program: Program<Params>,
    model: Model,
}

impl StdProgram {
    pub fn new(model: Model, translater: Matrix4<f32>) -> Result<Self, JsValue> {
        let vert_shader = VertexShader::compile(include_str!("standard.vert"))?;
        let frag_shader = FragmentShader::compile(include_str!("standard.frag"))?;

        let mut program = Program::<Params>::new(vert_shader, frag_shader)?;

        let vert_vbo = VBO::with_data(&model.positions);
        program.params.position.attach_vbo(&vert_vbo);

        let colors_vbo = VBO::with_data(&model.colors);
        program.params.color.attach_vbo(&colors_vbo);

        let ibo = IBO::with_data(&model.indexes);
        ibo.bind();

        program.params.mvp_matrix.set_value(translater);

        Ok(StdProgram { program, model })
    }

    pub fn set_translater(&mut self, translater: Matrix4<f32>) {
        self.program.params.mvp_matrix.set_value(translater);
    }

    pub fn render(&self) {
        context::with(|ctx| {
            ctx.draw_elements_with_i32(
                Context::TRIANGLES,
                self.model.indexes.as_ref().len() as i32,
                Context::UNSIGNED_SHORT,
                0,
            )
        })
    }
}

struct Params {
    position: Attribute<Vec3<f32>>,
    color: Attribute<Vec4<f32>>,
    mvp_matrix: Uniform<Mat4<f32>>,
}

impl ParamsBase for Params {
    fn from_visitor<'a>(visitor: ParamsVisitor<'a>) -> Result<Self, JsValue> {
        Ok(Params {
            position: visitor.visit_attr("position")?,
            color: visitor.visit_attr("color")?,
            mvp_matrix: visitor.visit_uniform("mvpMatrix")?,
        })
    }
}
