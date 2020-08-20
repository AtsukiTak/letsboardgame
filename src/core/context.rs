use super::program::GlProgram;
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
    pub fn clear_color_and_depth(&self, color: (f32, f32, f32, f32), depth: f32) {
        let (r, g, b, a) = color;
        self.gl.clear_color(r, g, b, a);
        self.gl.clear_depth(depth);
        self.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    }

    /// カリングを有効化する
    pub fn enable_culling(&self) {
        self.gl.enable(GL::CULL_FACE)
    }

    /// 深度テストを有効化する
    pub fn enable_depth_test(&self, depth_func: DepthFunc) {
        self.gl.enable(GL::DEPTH_TEST);
        self.gl.depth_func(depth_func.to_gl());
    }

    /// ブレンディングを有効化する
    pub fn enable_blending(&self, src_fac: BlendFactor, dst_fac: BlendFactor) {
        self.gl.enable(GL::BLEND);
        self.gl.blend_func(src_fac.to_gl(), dst_fac.to_gl());
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

pub enum DepthFunc {
    /// never pass
    Never,
    /// pass if the incoming value is less than the depth buffer value
    Less,
    /// pass if the incoming value equals to the depth buffer value
    Equal,
    /// pass if the incoming value is less than or equal to the depth buffer value
    LEqual,
    /// pass if the incoming value is greater than the depth buffer value
    Greater,
    /// pass if the incoming value is not equal to the depth buffer value
    NotEqual,
    /// pass if the incoming value is greater than or equal to the depth buffer value
    GEqual,
    /// always pass
    Always,
}

impl DepthFunc {
    pub fn to_gl(&self) -> u32 {
        match self {
            DepthFunc::Never => GL::NEVER,
            DepthFunc::Less => GL::LESS,
            DepthFunc::Equal => GL::EQUAL,
            DepthFunc::LEqual => GL::LEQUAL,
            DepthFunc::Greater => GL::GREATER,
            DepthFunc::NotEqual => GL::NOTEQUAL,
            DepthFunc::GEqual => GL::GEQUAL,
            DepthFunc::Always => GL::ALWAYS,
        }
    }
}

pub enum BlendFactor {
    /// (0, 0, 0, 0)
    Zero,
    /// (1, 1, 1, 1)
    One,
    /// (Rs, Gs, Bs, As)
    SrcColor,
    /// (Rd, Gd, Bd, Ad)
    DstColor,
    /// (1 - Rs, 1 - Gs, 1 - Bs, 1 - As)
    OneMinusSrcColor,
    /// (1 - Rd, 1 - Gd, 1 - Bd, 1 - Ad)
    OneMinusDstColor,
    /// (As, As, As, As)
    SrcAlpha,
    /// (Ad, Ad, Ad, Ad)
    DstAlpha,
    /// (1 - As, 1 - As, 1 - As, 1 - As)
    OneMinusSrcAlpha,
    /// (1 - Ad, 1 - Ad, 1 - Ad, 1 - Ad)
    OneMinusDstAlpha,
    /// (Rc, Gc, Bc, Ac)
    ConstantColor,
    /// (1 - Rc, 1 - Gc, 1 - Bc, 1 - Ac)
    OneMinusConstantColor,
    /// (Ac, Ac, Ac, Ac)
    ConstantAlpha,
    /// (1 - Ac, 1 - Ac, 1 - Ac, 1 - Ac)
    OneMinusConstantAlpha,
    /// (min(As, 1 - Ad), min(As, 1 - Ad), min(As, 1 - Ad), min(As, 1 - Ad))
    SrcAlphaSaturate,
}

impl BlendFactor {
    pub fn to_gl(&self) -> u32 {
        use BlendFactor::*;
        match self {
            Zero => GL::ZERO,
            One => GL::ONE,
            SrcColor => GL::SRC_COLOR,
            DstColor => GL::DST_COLOR,
            OneMinusSrcColor => GL::ONE_MINUS_SRC_COLOR,
            OneMinusDstColor => GL::ONE_MINUS_DST_COLOR,
            SrcAlpha => GL::SRC_ALPHA,
            DstAlpha => GL::DST_ALPHA,
            OneMinusSrcAlpha => GL::ONE_MINUS_SRC_ALPHA,
            OneMinusDstAlpha => GL::ONE_MINUS_DST_ALPHA,
            ConstantColor => GL::CONSTANT_COLOR,
            OneMinusConstantColor => GL::ONE_MINUS_CONSTANT_COLOR,
            ConstantAlpha => GL::CONSTANT_ALPHA,
            OneMinusConstantAlpha => GL::ONE_MINUS_CONSTANT_ALPHA,
            SrcAlphaSaturate => GL::SRC_ALPHA_SATURATE,
        }
    }
}
