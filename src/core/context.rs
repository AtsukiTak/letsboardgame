use super::color::Color;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};
use wasm_bindgen::{JsCast as _, JsValue};
use web_sys::WebGlRenderingContext as GL;

pub struct Context {
    gl: GL,
}

thread_local! {
    static GLOBAL_CONTEXT_CELL: RefCell<Option<Context>> = RefCell::new(None);
}

pub fn initialize(canvas: web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
    let gl = canvas.get_context("webgl")?.unwrap().dyn_into::<GL>()?;

    let context = Context { gl };

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

impl Context {
    pub fn clear_color(&self, color: &Color) {
        let (r, g, b, a) = color.to_f32();
        self.gl.clear_color(r, g, b, a)
    }

    pub fn enable(&self, cap: u32) {
        self.gl.enable(cap)
    }

    pub fn disable(&self, cap: u32) {
        self.gl.disable(cap)
    }

    pub fn enable_culling(&self) {
        self.enable(GL::CULL_FACE)
    }

    pub fn disable_culling(&self) {
        self.disable(GL::CULL_FACE)
    }

    pub fn enable_depth_test(&self) {
        self.enable(GL::DEPTH_TEST)
    }

    pub fn disable_depth_test(&self) {
        self.disable(GL::DEPTH_TEST)
    }
}

impl Deref for Context {
    type Target = GL;

    fn deref(&self) -> &GL {
        &self.gl
    }
}

impl DerefMut for Context {
    fn deref_mut(&mut self) -> &mut GL {
        &mut self.gl
    }
}
