use crate::event::EventStream;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, Node};

pub struct Canvas {
    canvas: HtmlCanvasElement,
}

impl Canvas {
    pub fn full_page() -> Result<Self, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();

        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        let body = document.body().unwrap();

        canvas.set_width(body.client_width() as u32);
        canvas.set_height(body.client_height() as u32);

        AsRef::<Node>::as_ref(&body).append_child(&canvas)?;

        Ok(Canvas::from_element(canvas))
    }

    pub fn from_element_id(id: &str) -> Option<Self> {
        let element = web_sys::window()?
            .document()?
            .get_element_by_id(id)?
            .dyn_into()
            .ok()?;
        Some(Canvas::from_element(element))
    }

    pub fn from_element(canvas: HtmlCanvasElement) -> Self {
        Canvas { canvas }
    }

    pub fn event_stream(&self) -> EventStream {
        EventStream::listen(&self.canvas)
    }
}

impl AsRef<HtmlCanvasElement> for Canvas {
    fn as_ref(&self) -> &HtmlCanvasElement {
        &self.canvas
    }
}
