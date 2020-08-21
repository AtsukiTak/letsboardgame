use cgmath::{vec3, Rad};
use napier::{meshes, Color, Light, Object, Renderer, Texture};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    let mut renderer = Renderer::new("canvas")?;

    let image = image::load_from_memory(include_bytes!("../myself.png")).unwrap();
    let texture = Texture::with_image_low(&image.into_rgba())?;
    let rect_mesh = meshes::rect_with_texture(4.0, 4.0, Color::rgba(255, 255, 255, 0.0), texture);
    let rect_obj = Object::new(rect_mesh);
    rect_obj.transform.rotate.axis.set(0.0, 1.0, 1.0);

    // カメラの設定
    renderer.camera.pos.z = 20.0;
    // 0.0, 0.0, 1.0 にすると何も映らなくなる...
    renderer.camera.up = vec3(0.0, 1.0, 0.0);

    // ライトの設定
    renderer.scene.light = Some(Light::point(0.0, 0.0, 10.0));

    renderer.scene.add(&rect_obj);

    renderer
        .start_rendering_loop(60, |_scene, _camera| {
            rect_obj.transform.rotate.angle.add(Rad(0.02));
        })
        .await;

    Ok(())
}
