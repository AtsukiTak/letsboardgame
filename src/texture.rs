use image::RgbaImage;
use wasm_bindgen::JsValue;

pub use napier_webgl::texture::{GlTexture, MagMethod, MinMethod};

#[derive(Debug, PartialEq)]
pub struct Texture {
    pub(crate) gl: GlTexture,
}

impl Texture {
    pub fn with_image_high(image: &RgbaImage) -> Result<Texture, JsValue> {
        let gl = GlTexture::new();
        gl.bind();
        gl.attach_img(&image, image.width() as i32, image.height() as i32)?;
        gl.generate_mipmap();
        gl.set_minify_filter(MinMethod::NearestMipmapLinear);
        gl.set_magnify_filter(MagMethod::Linear);
        gl.unbind();
        Ok(Texture { gl })
    }

    pub fn with_image_low(image: &RgbaImage) -> Result<Texture, JsValue> {
        let gl = GlTexture::new();
        gl.bind();
        gl.attach_img(&image, image.width() as i32, image.height() as i32)?;
        gl.set_minify_filter(MinMethod::Nearest);
        gl.set_magnify_filter(MagMethod::Nearest);
        gl.unbind();
        Ok(Texture { gl })
    }
}
