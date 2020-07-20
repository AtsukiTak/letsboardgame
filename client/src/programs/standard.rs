use crate::core::{
    buffers::{IBO, VBO},
    context::{self, Context},
    program::Program,
    shader::{Attribute, FragmentShader, ParamsBase, ParamsVisitor, Uniform, VertexShader},
    types::{Mat4, Vec3, Vec4},
};
use crate::models::Model;
use cgmath::{prelude::*, Matrix4, Vector3, Vector4};
use wasm_bindgen::prelude::*;

pub struct StdProgram {
    program: Program<Params>,
    model: Model,
}

impl StdProgram {
    pub fn new(
        model: Model,
        translater: Matrix4<f32>,
        light_dir: Vector3<f32>,
    ) -> Result<Self, JsValue> {
        let vert_shader = VertexShader::compile(include_str!("standard.vert"))?;
        let frag_shader = FragmentShader::compile(include_str!("standard.frag"))?;

        let mut program = Program::<Params>::new(vert_shader, frag_shader)?;

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

        // "mvpMatrix" uniformの設定
        program.params.mvp_matrix.set_value(translater);

        // "invMatrix" uniformの設定
        let inv_translater = translater.invert().unwrap();
        program.params.inv_matrix.set_value(inv_translater);

        // "lightDirection" uniformの設定
        program.params.light_direction.set_value(light_dir);

        Ok(StdProgram { program, model })
    }

    pub fn set_translater(&mut self, translater: Matrix4<f32>) {
        self.program.params.mvp_matrix.set_value(translater);

        let inv_translater = translater.invert().unwrap();
        self.program.params.inv_matrix.set_value(inv_translater);
    }

    pub fn set_ambient_color(&mut self, ambient_color: Vector4<f32>) {
        self.program.params.ambient_color.set_value(ambient_color);
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
    normal: Attribute<Vec3<f32>>,
    color: Attribute<Vec4<f32>>,
    mvp_matrix: Uniform<Mat4<f32>>,
    inv_matrix: Uniform<Mat4<f32>>,
    light_direction: Uniform<Vector3<f32>>,
    ambient_color: Uniform<Vector4<f32>>,
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
            ambient_color: visitor.visit_uniform("ambientColor")?,
        })
    }
}
