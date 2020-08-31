use cgmath::{vec3, Rad};
use napier::{meshes, window::Canvas, Camera, Color, Light, Object, Renderer, Scene, Texture};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    web_logger::init();

    let canvas = Canvas::from_element_id("canvas").unwrap();

    napier::init(&canvas).unwrap();

    let image = image::load_from_memory(include_bytes!("../myself.png")).unwrap();
    let texture = Texture::with_image_low(&image.into_rgba())?;
    let rect_obj = Object::new(meshes::rect_with_texture(
        4.0,
        4.0,
        Color::rgba(255, 255, 255, 1.0),
        texture,
    ));
    rect_obj.transform.rotate.axis.set(0.0, 1.0, 1.0);

    let transparent_rect = Object::new(meshes::rect(6.0, 6.0, Color::rgba(100, 100, 100, 0.3)));
    transparent_rect.transform.pos.z.set(-1.0);

    // カメラの設定
    let mut camera = Camera::new();
    camera.pos.z = 20.0;
    // 0.0, 0.0, 1.0 にすると何も映らなくなる(cameraの向きと同じになっちゃう)
    camera.up = vec3(0.0, 1.0, 0.0);

    // シーンの設定
    let mut scene = Scene::new();
    scene.light = Some(Light::point(0.0, 0.0, 10.0));
    scene.add(&rect_obj);
    scene.add(&transparent_rect);

    // レンダリング
    let mut renderer = Renderer::new()?;
    loop {
        rect_obj.transform.rotate.angle.add(Rad(0.02));

        renderer.render(&scene, &camera);

        gloo_timers::future::TimeoutFuture::new(1000 / 60).await;
    }

    Ok(())
}
