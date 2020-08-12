use cgmath::{vec3, Rad};
use three_wasm::{
    core::{color::Color, context, texture::Texture, GL},
    light::Light,
    meshes,
    object::Object,
    renderer::Renderer,
};
use wasm_bindgen::{prelude::*, JsCast as _};

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    initialize()?;

    let image = image::load_from_memory(include_bytes!("../myself.png")).unwrap();
    let texture = Texture::with_image(&image.into_rgba())?;
    let rect_mesh = meshes::rect_with_texture(4.0, 4.0, Color::rgba(255, 255, 255, 0.0), texture);
    let rect_obj = Object::new(rect_mesh);
    rect_obj.transform.rotate.axis.set(0.0, 1.0, 1.0);

    let mut renderer = Renderer::new()?;

    // カメラの設定
    renderer.camera.pos.z = 20.0;
    // 0.0, 0.0, 1.0 にすると何も映らなくなる...
    renderer.camera.up = vec3(0.0, 1.0, 0.0);

    // ライトの設定
    renderer.scene.light = Some(Light::point(0.0, 0.0, 10.0));

    renderer.scene.add(&rect_obj);

    loop {
        clear();

        rect_obj.transform.rotate.angle.add(Rad(0.02));

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

    context::with(|ctx| {
        ctx.enable_culling();
        ctx.enable_depth_test();
        ctx.depth_func(GL::LEQUAL);
    });

    Ok(())
}

fn clear() {
    context::with(|ctx| {
        ctx.clear_depth(1.0);
        ctx.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    });
}
