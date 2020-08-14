use super::{color::Color, program::GlProgram};
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};
use wasm_bindgen::{JsCast as _, JsValue};
use web_sys::WebGlRenderingContext as GL;

pub struct Context {
    gl: GL,
    enabled_vertex_attrib_locations: Vec<u32>,
}

thread_local! {
    static GLOBAL_CONTEXT_CELL: RefCell<Option<Context>> = RefCell::new(None);
}

pub fn initialize(canvas: web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
    let gl = canvas.get_context("webgl")?.unwrap().dyn_into::<GL>()?;

    let context = Context {
        gl,
        enabled_vertex_attrib_locations: Vec::new(),
    };

    GLOBAL_CONTEXT_CELL.with(|cell| cell.replace(Some(context)));

    Ok(())
}

/// panic if uninitialized
pub fn with<F, T>(func: F) -> T
where
    F: FnOnce(&mut Context) -> T,
{
    GLOBAL_CONTEXT_CELL.with(|cell| func(cell.borrow_mut().as_mut().unwrap()))
}

impl Context {
    pub fn clear_color_and_depth(&self, color: &Color, depth: f32) {
        let (r, g, b, a) = color.to_f32();
        self.gl.clear_color(r, g, b, a);
        self.gl.clear_depth(depth);
        self.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    }

    pub fn enable(&self, cap: u32) {
        self.gl.enable(cap)
    }

    pub fn enable_culling(&self) {
        self.enable(GL::CULL_FACE)
    }

    pub fn enable_depth_test(&self) {
        self.enable(GL::DEPTH_TEST)
    }

    /// 指定されたGlProgramに切り替える
    /// WebGLのAPI呼び出しとしては、以下の3つのAPIを呼び出している
    ///
    /// - use_program
    ///   - programの有効化のため
    /// - enable_vertex_attrib_array
    ///   - 新たに使用するvertex attributeの有効のため
    /// - disable_vertex_attrib_array
    ///   - 使用しなくなったvertex attributeの無効化のため
    pub fn switch_program<P>(&mut self, program: &GlProgram<P>) {
        let new_attrib = program.vertex_attrib_locations();

        // 新しいprogramで使用しないvertex_attribの無効化
        while let Some((idx, loc)) = self
            .enabled_vertex_attrib_locations
            .iter()
            .enumerate()
            .find(|(_, loc)| !new_attrib.contains(loc))
        {
            self.gl.disable_vertex_attrib_array(*loc);
            self.enabled_vertex_attrib_locations.swap_remove(idx);
        }

        // 新しいprogramで新たに使用するvertex_attribの有効化
        while let Some(loc) = new_attrib
            .iter()
            .find(|loc| !self.enabled_vertex_attrib_locations.contains(loc))
        {
            self.gl.enable_vertex_attrib_array(*loc);
            self.enabled_vertex_attrib_locations.push(*loc);
        }

        // programの有効化
        self.gl.use_program(Some(&program.program))
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
