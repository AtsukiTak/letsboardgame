mod context;
mod shader;
mod start;
mod vbo;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    start::start()
}
