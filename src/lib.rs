pub mod core;
pub mod models;
pub mod original;
pub mod programs;
mod start;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    start::start().await
}