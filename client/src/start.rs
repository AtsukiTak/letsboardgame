use crate::{
    core::context::{self, Context},
    models::torus,
    programs::standard::StdProgram,
};
use cgmath::{prelude::*, vec3, vec4, Deg, Matrix4, Point3};
use wasm_bindgen::{prelude::*, JsCast as _};

pub async fn start() -> Result<(), JsValue> {
    initialize()?;

    let model = torus(1.0, 64, 2.0, 64);
    // let model = sphere(32, 32, 2.0);

    let mut program = StdProgram::new(model)?;
    let params = program.params_mut();
    params.light_direction.set_value(vec3(-0.5, 0.5, 0.5));
    params.ambient_color.set_value(vec4(0.1, 0.1, 0.1, 0.1));
    params.eye_direction.set_value(vec3(0.0, 0.0, 20.0));

    let vp_matrix = vp_matrix();
    let mut frame = 1;
    loop {
        clear();

        let m_matrix = m_matrix(frame);
        let params = program.params_mut();
        params.mvp_matrix.set_value(vp_matrix * m_matrix);
        params.inv_matrix.set_value(m_matrix.invert().unwrap());

        program.render();

        frame = frame + 1;

        gloo_timers::future::TimeoutFuture::new(1000 / 30).await;
    }

    Ok(())
}

fn initialize() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    context::initialize(canvas)?;

    context::enable_culling();
    context::enable_depth_test();
    context::with(|ctx| ctx.depth_func(Context::LEQUAL));

    Ok(())
}

fn clear() {
    context::with(|ctx| {
        ctx.clear_color(0.0, 0.0, 0.0, 1.0);
        ctx.clear_depth(1.0);
        ctx.clear(Context::COLOR_BUFFER_BIT | Context::DEPTH_BUFFER_BIT);
    });
}

fn vp_matrix() -> Matrix4<f32> {
    // ビュー座標変換行列
    let v_mat = Matrix4::look_at(
        Point3::new(0.0, 0.0, 20.0), // カメラの位置
        Point3::new(0.0, 0.0, 0.0),  // 視点の中央
        vec3(0.0, 1.0, 0.0),         // 上方向のベクトル
    );

    // プロジェクション座標変換行列
    let p_mat = cgmath::perspective(
        Deg(45.0), // 画角
        1.0,       // アスペクト比
        0.1,       // どれくらい近くまでカメラに写すか
        100.0,     // どれくらい遠くまでカメラに写すか
    );

    p_mat * v_mat
}

fn m_matrix(frame: usize) -> Matrix4<f32> {
    let angle = Deg((frame % 360) as f32);
    let axis = vec3(0.0, 1.0, 1.0).normalize();
    Matrix4::from_axis_angle(axis, angle)
}

/*
 * ========
 * Test用
 * ========
 */
use crate::original;

#[allow(dead_code)]
pub async fn start_test() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    context::initialize(canvas)?;

    context::enable_culling();
    context::enable_depth_test();
    context::with(|ctx| ctx.depth_func(Context::LEQUAL));

    let model = torus(1.0, 32, 2.0, 32);
    let index_len = model.indexes.as_ref().len();

    let mut program = StdProgram::new(model)?;
    let params = program.params_mut();
    params.light_direction.set_value(vec3(-0.5, 0.5, 0.5));
    params.ambient_color.set_value(vec4(0.1, 0.1, 0.1, 0.1));
    params.eye_direction.set_value(vec3(0.0, 0.0, 20.0));

    context::with(|ctx| {
        original::rendering_loop(
            ctx,
            index_len,
            &program.program.params.mvp_matrix.location,
            &program.program.params.inv_matrix.location,
        )
    });

    Ok(())
}
