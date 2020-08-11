use crate::{
    camera::Camera,
    core::context::{self, Context},
    light::Light,
    object::Object,
    programs::{BasicParams, BasicProgram, TextureProgram},
    scene::Scene,
};
use cgmath::prelude::*;
use wasm_bindgen::JsValue;

pub struct Renderer {
    basic_program: BasicProgram,
    texture_program: TextureProgram,
    pub scene: Scene,
    pub camera: Camera,
}

impl Renderer {
    pub fn new() -> Result<Self, JsValue> {
        Ok(Renderer {
            basic_program: BasicProgram::gouraud()?,
            texture_program: TextureProgram::gouraud()?,
            scene: Scene::new(),
            camera: Camera::new(),
        })
    }

    pub fn render(&mut self) {
        // 背景色の設定
        context::clear_color(&self.scene.background);

        for object in self.scene.objects() {
            if object.mesh.texture.is_some() {
                render_texture_object(&mut self.texture_program, &self.scene, &self.camera, object);
            } else {
                render_basic_object(&mut self.basic_program, &self.scene, &self.camera, object);
            }
        }
    }
}

fn render_basic_object(
    program: &mut BasicProgram,
    scene: &Scene,
    camera: &Camera,
    object: &Object,
) {
    program.use_program();

    set_basic_uniforms(program.params_mut(), scene, camera, object);

    set_basic_attrs(program.params(), object);

    context::with(|ctx| {
        ctx.draw_elements_with_i32(
            Context::TRIANGLES,
            object.mesh.index_len,
            Context::UNSIGNED_SHORT,
            0,
        );
    })
}

fn render_texture_object(
    program: &mut TextureProgram,
    scene: &Scene,
    camera: &Camera,
    object: &Object,
) {
    program.use_program();

    set_basic_uniforms(program.params_mut().as_mut(), scene, camera, object);

    set_basic_attrs(program.params().as_ref(), object);

    // テクスチャの関連の設定
    let texture = object.mesh.texture.as_ref().unwrap();

    // texCoord attributeの設定
    program.params().tex_coord.attach_vbo(&texture.coord);

    // テクスチャユニットの設定
    texture.data.attach_unit(0);
    program.params_mut().texture.set_value(0);

    context::with(|ctx| {
        ctx.draw_elements_with_i32(
            Context::TRIANGLES,
            object.mesh.index_len,
            Context::UNSIGNED_SHORT,
            0,
        );
    })
}

fn set_basic_uniforms(params: &mut BasicParams, scene: &Scene, camera: &Camera, object: &Object) {
    // ambient_color の設定
    params
        .ambient_color
        .set_value(scene.ambient_color.to_f32_vec4());

    // eye_directionの設定
    let eye_direction = camera.look_at - camera.pos;
    params.eye_direction.set_value(eye_direction);

    // lightの設定
    match scene.light {
        Some(Light::Directional(ref light)) => {
            params.light_type.set_value(1);
            params.light_val.set_value(light.dir);
        }
        Some(Light::Point(ref light)) => {
            params.light_type.set_value(2);
            params.light_val.set_value(light.pos);
        }
        None => {
            params.light_type.set_value(0);
        }
    }

    // カメラ, Transform周りの設定
    let vp_matrix = camera.matrix();
    let m_matrix = object.transform.matrix();
    params.m_matrix.set_value(m_matrix);
    params.mvp_matrix.set_value(vp_matrix * m_matrix);
    params.inv_m_matrix.set_value(m_matrix.invert().unwrap());
}

fn set_basic_attrs(params: &BasicParams, object: &Object) {
    // 各attribute変数の設定
    let mesh = &object.mesh;

    // "position" attributeの設定
    params.position.attach_vbo(&mesh.positions_vbo);

    // "normal" attributeの設定
    params.normal.attach_vbo(&mesh.normals_vbo);

    // "color" attributeの設定
    params.color.attach_vbo(&mesh.colors_vbo);

    // Index Bufferの設定
    mesh.indexes_ibo.bind();
}
