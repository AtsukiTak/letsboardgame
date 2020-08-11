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

        self.program.use_program();

        // ambient_color の設定
        self.program
            .params_mut()
            .ambient_color
            .set_value(self.scene.ambient_color.to_f32_vec4());

        // eye_directionの設定
        let eye_direction = self.camera.look_at - self.camera.pos;
        self.program
            .params_mut()
            .eye_direction
            .set_value(eye_direction);

        // lightの設定
        match self.scene.light {
            Some(Light::Directional(ref light)) => {
                let params = self.program.params_mut();
                params.light_type.set_value(1);
                params.light_val.set_value(light.dir);
            }
            Some(Light::Point(ref light)) => {
                let params = self.program.params_mut();
                params.light_type.set_value(2);
                params.light_val.set_value(light.pos);
            }
            None => {
                let params = self.program.params_mut();
                params.light_type.set_value(0);
            }
        }

        // カメラ周りの設定
        let vp_matrix = self.camera.matrix();

        for object in self.scene.objects() {
            // 各uniform変数の設定
            let m_matrix = object.transform.matrix();
            let params = self.program.params_mut();
            params.m_matrix.set_value(m_matrix);
            params.mvp_matrix.set_value(vp_matrix * m_matrix);
            params.inv_m_matrix.set_value(m_matrix.invert().unwrap());

            self.render_object(object);
        }
    }

    fn render_object(&self, object: &Object) {
        let mesh = &object.mesh;

        // "position" attributeの設定
        self.program
            .params()
            .position
            .attach_vbo(&mesh.positions_vbo);

        // "normal" attributeの設定
        self.program.params().normal.attach_vbo(&mesh.normals_vbo);

        // "color" attributeの設定
        self.program.params().color.attach_vbo(&mesh.colors_vbo);

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
}
