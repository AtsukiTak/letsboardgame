use wasm_bindgen::{prelude::*, JsCast as _};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<web_sys::WebGlRenderingContext>()?;

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT);

    Ok(())
}
