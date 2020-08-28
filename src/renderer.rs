use crate::{
    camera::Camera,
    light::Light,
    object::Object,
    programs::{BasicParams, BasicProgram, TextureProgram},
    scene::Scene,
    window::Canvas,
};
use cgmath::prelude::*;
use napier_webgl::{
    context::{self, BlendFactor, DepthFunc},
    texture::GlTextureUnit,
};
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;

pub struct Renderer {
    basic_program: BasicProgram,
    texture_program: TextureProgram,
    pub canvas: Canvas,
    pub scene: Scene,
    pub camera: Camera,
}

impl Renderer {
    /// このライブラリを利用するときのエントリーポイント
    pub fn new(canvas: Canvas) -> Result<Self, JsValue> {
        context::initialize(canvas.as_ref())?;

        context::with(|ctx| {
            ctx.enable_culling();
            ctx.enable_depth_test(DepthFunc::LEqual);
            // 透過処理のブレンディングを有効化
            ctx.enable_separate_blending(
                BlendFactor::SrcAlpha,         // src_rgb
                BlendFactor::OneMinusSrcAlpha, // dst_rgb
                BlendFactor::One,              // src_alpha
                BlendFactor::One,              // dst_alpha
            );
        });

        Ok(Renderer {
            basic_program: BasicProgram::gouraud()?,
            texture_program: TextureProgram::phong()?,
            canvas,
            scene: Scene::new(),
            camera: Camera::new(),
        })
    }

    pub async fn start_rendering_loop(
        mut self,
        fps: u32,
        mut before_frame: impl FnMut(&mut Scene, &mut Camera),
    ) {
        loop {
            before_frame(&mut self.scene, &mut self.camera);

            self.render();

            gloo_timers::future::TimeoutFuture::new(1000 / fps).await;
        }
    }

    fn render(&mut self) {
        context::with(|ctx| {
            // 背景色と深度の設定
            ctx.clear_color_and_depth(self.scene.background.to_f32(), 1.0);
        });

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
    program.switch();

    set_basic_uniforms(program.params_mut(), scene, camera, object);

    set_basic_attrs(program.params(), object);

    context::with(|ctx| {
        ctx.draw_elements_with_i32(GL::TRIANGLES, object.mesh.index_len, GL::UNSIGNED_SHORT, 0);
    })
}

fn render_texture_object(
    program: &mut TextureProgram,
    scene: &Scene,
    camera: &Camera,
    object: &Object,
) {
    program.switch();

    set_basic_uniforms(program.params_mut().as_mut(), scene, camera, object);

    set_basic_attrs(program.params().as_ref(), object);

    // テクスチャの関連の設定
    let texture = object.mesh.texture.as_ref().unwrap();

    // texCoord attributeの設定
    program.params().tex_coord.attach_vbo(&texture.coord);

    // テクスチャユニットの設定
    texture.data.gl.bind();
    let unit = GlTextureUnit::Unit0;
    unit.activate();
    program.params_mut().texture.set_value(unit);

    context::with(|ctx| {
        ctx.draw_elements_with_i32(GL::TRIANGLES, object.mesh.index_len, GL::UNSIGNED_SHORT, 0);
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
