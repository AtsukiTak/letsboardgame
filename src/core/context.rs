use super::color::Color;
use std::cell::RefCell;
use wasm_bindgen::{JsCast as _, JsValue};

pub type Context = web_sys::WebGlRenderingContext;

thread_local! {
    static GLOBAL_CONTEXT_CELL: RefCell<Option<Context>> = RefCell::new(None);
}

pub fn initialize(canvas: web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<Context>()?;

    GLOBAL_CONTEXT_CELL.with(|cell| cell.replace(Some(context)));

    Ok(())
}

/// panic if uninitialized
pub fn with<F, T>(func: F) -> T
where
    F: FnOnce(&Context) -> T,
{
    GLOBAL_CONTEXT_CELL.with(|cell| func(cell.borrow().as_ref().unwrap()))
}

pub fn clear_color(color: &Color) {
    with(|ctx| ctx.clear_color(color.r as f32, color.g as f32, color.b as f32, color.a))
}

pub fn enable(cap: u32) {
    with(|ctx| ctx.enable(cap))
}

pub fn disable(cap: u32) {
    with(|ctx| ctx.disable(cap))
}

pub fn enable_culling() {
    enable(Context::CULL_FACE)
}

pub fn disable_culling() {
    disable(Context::CULL_FACE)
}

pub fn enable_depth_test() {
    enable(Context::DEPTH_TEST)
}

pub fn disable_depth_test() {
    disable(Context::DEPTH_TEST)
}
