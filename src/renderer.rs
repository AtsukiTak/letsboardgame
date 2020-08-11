use crate::{
    camera::Camera,
    core::context::{self, Context},
    light::Light,
    object::Object,
    programs::BasicProgram,
    scene::Scene,
};
use cgmath::prelude::*;
use wasm_bindgen::JsValue;

pub struct Renderer {
    program: BasicProgram,
    pub scene: Scene,
    pub camera: Camera,
}

impl Renderer {
    pub fn new() -> Result<Self, JsValue> {
        Ok(Renderer {
            program: BasicProgram::gouraud()?,
            scene: Scene::new(),
            camera: Camera::new(),
        })
    }

    pub fn render(&mut self) {
        // 背景色の設定
        context::clear_color(&self.scene.background);

        for object in self.scene.objects() {
            render_simple_object(&mut self.program, &self.scene, &self.camera, object);
        }
    }
}

fn render_simple_object(
    program: &mut BasicProgram,
    scene: &Scene,
    camera: &Camera,
    object: &Object,
) {
    program.use_program();

    // ambient_color の設定
    program
        .params_mut()
        .ambient_color
        .set_value(scene.ambient_color.to_f32_vec4());

    // eye_directionの設定
    let eye_direction = camera.look_at - camera.pos;
    program.params_mut().eye_direction.set_value(eye_direction);

    // lightの設定
    match scene.light {
        Some(Light::Directional(ref light)) => {
            let params = program.params_mut();
            params.light_type.set_value(1);
            params.light_val.set_value(light.dir);
        }
        Some(Light::Point(ref light)) => {
            let params = program.params_mut();
            params.light_type.set_value(2);
            params.light_val.set_value(light.pos);
        }
        None => {
            let params = program.params_mut();
            params.light_type.set_value(0);
        }
    }

    // カメラ, Transform周りの設定
    let vp_matrix = camera.matrix();
    let m_matrix = object.transform.matrix();
    program.params_mut().m_matrix.set_value(m_matrix);
    program
        .params_mut()
        .mvp_matrix
        .set_value(vp_matrix * m_matrix);
    program
        .params_mut()
        .inv_m_matrix
        .set_value(m_matrix.invert().unwrap());

    // 各attribute変数の設定
    let mesh = &object.mesh;

    // "position" attributeの設定
    program.params().position.attach_vbo(&mesh.positions_vbo);

    // "normal" attributeの設定
    program.params().normal.attach_vbo(&mesh.normals_vbo);

    // "color" attributeの設定
    program.params().color.attach_vbo(&mesh.colors_vbo);

    // Index Bufferの設定
    mesh.indexes_ibo.bind();

    context::with(|ctx| {
        ctx.draw_elements_with_i32(
            Context::TRIANGLES,
            mesh.index_len,
            Context::UNSIGNED_SHORT,
            0,
        );
    })
}
