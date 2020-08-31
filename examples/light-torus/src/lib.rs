use cgmath::{vec3, Rad};
use futures::stream::StreamExt as _;
use napier::{
    meshes,
    window::{event::Event, Canvas},
    Camera, Color, Light, Object, Renderer, Scene, Texture,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    web_logger::init();

    let canvas = Canvas::from_element_id("canvas").unwrap();

    napier::init(&canvas).unwrap();

    let objects = Objects::new()?;

    // カメラの設定
    let mut camera = Camera::new();
    camera.pos.z = 20.0;
    // 0.0, 0.0, 1.0 にすると何も映らなくなる(cameraの向きと同じになっちゃう)
    camera.up = vec3(0.0, 1.0, 0.0);

    // シーンの設定
    let mut scene = Scene::new();
    scene.light = Some(Light::point(0.0, 0.0, 10.0));
    scene.add(&objects.texture);
    scene.add(&objects.transparent_rect);

    // イベントハンドラの設定
    let event_handler_fut = canvas.event_stream().for_each(|event| match event {
        Event::MouseMove(event) => {
            let x = event.x() as f32 - canvas.width() as f32 / 2.0;
            let y = event.y() as f32 - canvas.height() as f32 / 2.0;
            objects.texture.transform.rotate.axis.set(y, x, 0.0);
            futures::future::ready(())
        }
        _ => futures::future::ready(()),
    });
    futures::pin_mut!(event_handler_fut);

    // レンダリング
    let mut renderer = Renderer::new()?;
    let rendering_fut = async {
        loop {
            objects.texture.transform.rotate.angle.add(Rad(0.02));

            renderer.render(&scene, &camera);

            gloo_timers::future::TimeoutFuture::new(1000 / 60).await;
        }
    };
    futures::pin_mut!(rendering_fut);

    futures::future::select(event_handler_fut, rendering_fut).await;

    Ok(())
}

pub struct Objects {
    texture: Object,
    transparent_rect: Object,
}

impl Objects {
    pub fn new() -> Result<Self, JsValue> {
        let image = image::load_from_memory(include_bytes!("../myself.png")).unwrap();
        let texture = Texture::with_image_low(&image.into_rgba())?;
        let texture_obj = Object::new(meshes::rect_with_texture(
            4.0,
            4.0,
            Color::rgba(255, 255, 255, 1.0),
            texture,
        ));
        texture_obj.transform.rotate.axis.set(0.0, 1.0, 1.0);

        let transparent_rect = Object::new(meshes::rect(6.0, 6.0, Color::rgba(100, 100, 100, 0.3)));
        transparent_rect.transform.pos.z.set(-1.0);

        Ok(Objects {
            texture: texture_obj,
            transparent_rect,
        })
    }
}
