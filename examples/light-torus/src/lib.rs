use cgmath::{vec3, Rad};
use three_wasm::{
    core::context::{self, Context},
    light::Light,
    meshes,
    object::Object,
    renderer::Renderer,
};
use wasm_bindgen::{prelude::*, JsCast as _};

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    initialize()?;

    let torus_mesh = meshes::torus(1.0, 64, 2.0, 64);
    let torus = Object::new(torus_mesh);
    torus.transform.pos.x.set(-5.0);
    torus.transform.rotate.axis.set(0.0, 1.0, 1.0);

    let sphere_mesh = meshes::sphere(32, 32, 2.0);
    let sphere = Object::new(sphere_mesh);
    sphere.transform.pos.x.set(5.0);

    let mut renderer = Renderer::new()?;

    // カメラの設定
    renderer.camera.pos.z = 20.0;
    // 0.0, 0.0, 1.0 にすると何も映らなくなる...
    renderer.camera.up = vec3(0.0, 1.0, 0.0);

    // ライトの設定
    renderer.scene.light = Some(Light::point(0.0, 0.0, 10.0));

    renderer.scene.add(&torus);
    renderer.scene.add(&sphere);

    loop {
        clear();

        torus.transform.rotate.angle.add(Rad(0.01));

        renderer.render();

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
    context::with(|ctx| ctx.depth_func(Context::LEQUAL));

    Ok(())
}

fn clear() {
    context::with(|ctx| {
        ctx.clear_depth(1.0);
        ctx.clear(Context::COLOR_BUFFER_BIT | Context::DEPTH_BUFFER_BIT);
    });
}
