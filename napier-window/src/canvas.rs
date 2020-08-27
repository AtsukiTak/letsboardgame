use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;

pub struct Canvas {
    canvas: HtmlCanvasElement,
}

impl Canvas {
    pub fn new() -> Result<Self, JsValue> {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")?
            .dyn_into()
            .unwrap();

        Ok(Canvas { canvas })
    }
}

impl AsRef<HtmlCanvasElement> for Canvas {
    fn as_ref(&self) -> &HtmlCanvasElement {
        &self.canvas
    }
}
