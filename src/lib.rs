mod camera;
mod color;
mod object;
mod renderer;
mod scene;
mod texture;

pub mod cell;
pub mod light;
pub mod meshes;
pub mod programs;

#[cfg(test)]
mod original;

pub use camera::Camera;
pub use color::Color;
pub use light::Light;
pub use meshes::Mesh;
pub use object::{Object, Transform};
pub use renderer::Renderer;
pub use scene::Scene;
pub use texture::Texture;

pub use napier_webgl as webgl;
pub use napier_window as window;

use wasm_bindgen::JsValue;

/// `Canvas` 以外のオブジェクトを作成する前にこの `init` 関数を呼び出さなければいけない
pub fn init(canvas: &window::Canvas) -> Result<(), JsValue> {
    webgl::context::initialize(canvas.as_ref())
}
