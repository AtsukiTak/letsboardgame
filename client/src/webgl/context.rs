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
