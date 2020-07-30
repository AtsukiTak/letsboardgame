use js_sys::Array;
use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation};

#[wasm_bindgen(module = "/src/original/main.js")]
extern "C" {
    pub fn start();
    pub fn start_inner(gl: &WebGlRenderingContext, index_len: usize, program: &WebGlProgram);
    pub fn rendering_loop(
        gl: &WebGlRenderingContext,
        indexLen: usize,
        mvpMatrixLoc: &WebGlUniformLocation,
        invMatrixLoc: &WebGlUniformLocation,
    );
}
