use super::context;
use std::marker::PhantomData;
use web_sys::WebGlRenderingContext as GL;

#[derive(Debug, PartialEq)]
pub struct VBO<T> {
    buf: web_sys::WebGlBuffer,
    _type: PhantomData<T>,
}

impl<T> VBO<T>
where
    T: AsRef<[f32]>,
{
    pub fn new() -> Self {
        context::with(|ctx| {
            // bufferの作成
            let buf = ctx.create_buffer().unwrap();

            VBO {
                buf,
                _type: PhantomData,
            }
        })
    }

    pub fn with_data(data: &T) -> Self {
        let vbo = VBO::new();
        vbo.set_data(data);
        vbo
    }

    pub fn bind(&self) {
        context::with(|ctx| {
            ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buf));
        })
    }

    pub fn unbind(&self) {
        context::with(|ctx| {
            ctx.bind_buffer(GL::ARRAY_BUFFER, None);
        })
    }

    pub fn set_data(&self, data: &T) {
        self.bind();

        context::with(|ctx| {
            let js_array = js_sys::Float32Array::from(data.as_ref());

            // bufferにデータをセット
            ctx.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_array, GL::STATIC_DRAW);
        });

        self.unbind();
    }
}

/// インデックスバッファーオブジェクト
/// 型パラメータ `T` は `Vec3<i16>` などを想定
#[derive(Debug, PartialEq)]
pub struct IBO<T> {
    buf: web_sys::WebGlBuffer,
    _type: PhantomData<T>,
}

impl<T> IBO<T>
where
    T: AsRef<[i16]>,
{
    pub fn new() -> Self {
        context::with(|ctx| {
            // bufferの作成
            let buf = ctx.create_buffer().unwrap();

            IBO {
                buf,
                _type: PhantomData,
            }
        })
    }

    pub fn with_data(data: &T) -> Self {
        let vbo = IBO::new();
        vbo.set_data(data);
        vbo
    }

    pub fn bind(&self) {
        context::with(|ctx| {
            ctx.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.buf));
        })
    }

    pub fn unbind(&self) {
        context::with(|ctx| {
            ctx.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);
        })
    }

    pub fn set_data(&self, data: &T) {
        self.bind();

        context::with(|ctx| {
            let js_array = js_sys::Int16Array::from(data.as_ref());

            // bufferにデータをセット
            ctx.buffer_data_with_array_buffer_view(
                GL::ELEMENT_ARRAY_BUFFER,
                &js_array,
                GL::STATIC_DRAW,
            );
        });

        self.unbind();
    }
}
