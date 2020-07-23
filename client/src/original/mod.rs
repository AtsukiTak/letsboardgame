use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/original/main.js")]
extern "C" {
    pub fn start();
}
