use crate::context::{self, Context};

pub struct VBO {
    buf: web_sys::WebGlBuffer,
}

impl VBO {
    pub fn new() -> Self {
        context::with(|ctx| {
            // bufferの作成
            let buf = ctx.create_buffer().unwrap();

            VBO { buf }
        })
    }

    pub fn with_data(data: &[f32]) -> Self {
        let vbo = VBO::new();
        vbo.set_data(data);
        vbo
    }

    pub fn bind(&self) {
        context::with(|ctx| {
            ctx.bind_buffer(Context::ARRAY_BUFFER, Some(&self.buf));
        })
    }

    pub fn unbind(&self) {
        context::with(|ctx| {
            ctx.bind_buffer(Context::ARRAY_BUFFER, None);
        })
    }

    pub fn set_data(&self, data: &[f32]) {
        self.bind();

        context::with(|ctx| {
            let js_array = js_sys::Float32Array::from(data);

            // bufferにデータをセット
            ctx.buffer_data_with_array_buffer_view(
                Context::ARRAY_BUFFER,
                &js_array,
                Context::STATIC_DRAW,
            );
        });

        self.unbind();
    }
}
