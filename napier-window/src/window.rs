use web_sys::HtmlCanvasElement;

pub struct Window {
    canvas: HtmlCanvasElement,
}

impl Window {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        Window { canvas }
    }
}
