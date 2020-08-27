use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;

pub struct Canvas {
    canvas: HtmlCanvasElement,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Result<Self, JsValue> {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")?
            .dyn_into()
            .unwrap();

        canvas.set_width(width);
        canvas.set_height(width);

        Ok(Canvas { canvas })
    }
}

impl AsRef<HtmlCanvasElement> for Canvas {
    fn as_ref(&self) -> &HtmlCanvasElement {
        &self.canvas
    }
}
