use crate::core::texture::GlTexture;
use image::RgbaImage;
use wasm_bindgen::JsValue;

pub use crate::core::texture::{MagMethod, MinMethod};

#[derive(Debug, PartialEq)]
pub struct Texture {
    pub(crate) gl: GlTexture,
}

impl Texture {
    pub fn with_image(image: &RgbaImage) -> Result<Texture, JsValue> {
        let gl = GlTexture::with_raw_image(&image, image.width() as i32, image.height() as i32)?;
        gl.set_minify_filter(MinMethod::NearestMipmapLinear);
        gl.set_magnify_filter(MagMethod::Linear);
        Ok(Texture { gl })
    }
}
