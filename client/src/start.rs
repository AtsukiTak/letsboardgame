use crate::webgl::context::{self, Context};
use crate::{models::torus, programs::StdProgram};
use cgmath::{prelude::*, Deg, Matrix4, Point3, Vector3};
use wasm_bindgen::{prelude::*, JsCast as _};

pub async fn start() -> Result<(), JsValue> {
    initialize()?;

    let model = torus(1.0, 32, 2.0, 32);

    let vp_matrix = vp_matrix();
    let translater = vp_matrix * m_matrix(0);

    let mut program = StdProgram::new(model, translater)?;

    let mut frame = 1;
    loop {
        clear();

        program.set_translater(vp_matrix * m_matrix(frame));
        program.render();

        frame = frame + 1;

        gloo_timers::future::TimeoutFuture::new(1000 / 60).await;
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
        Vector3::new(0.0, 1.0, 0.0), // 上方向のベクトル
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
    let axis = Vector3::new(0.0, 1.0, 1.0).normalize();
    Matrix4::from_axis_angle(axis, angle)
}
