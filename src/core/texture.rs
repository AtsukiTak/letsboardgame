use super::context::{self, Context};
use image::RgbaImage;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Texture {
    gl_texture: web_sys::WebGlTexture,
}

impl Texture {
    pub fn new() -> Texture {
        context::with(|ctx| Texture {
            gl_texture: ctx.create_texture().unwrap(),
        })
    }

    /// 画像データとともにTextureオブジェクトを初期化する
    pub fn with_image_parts(pixels: &[u8], width: i32, height: i32) -> Result<Texture, JsValue> {
        let tex = Texture::new();
        tex.bind();
        tex.attach_img(pixels, width, height)?;
        tex.generate_mipmap();
        tex.unbind();
        Ok(tex)
    }

    pub fn with_image(image: &RgbaImage) -> Result<Texture, JsValue> {
        Texture::with_image_parts(&image, image.width() as i32, image.height() as i32)
    }

    // テクスチャユニットを有効化し、そこにテクスチャをbindする
    pub fn attach_unit(&self, unit_id: u32) {
        self.active_unit(unit_id);
        self.bind();
    }

    fn bind(&self) {
        context::with(|ctx| ctx.bind_texture(Context::TEXTURE_2D, Some(&self.gl_texture)))
    }

    fn unbind(&self) {
        context::with(|ctx| ctx.bind_texture(Context::TEXTURE_2D, None))
    }

    fn attach_img(&self, pixels: &[u8], width: i32, height: i32) -> Result<(), JsValue> {
        context::with(|ctx| {
            ctx.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                Context::TEXTURE_2D,  // target
                0,                    // level
                Context::RGBA as i32, // internal format
                width,
                height,
                0,                      // border. Must be 0.
                Context::RGBA,          // format
                Context::UNSIGNED_BYTE, // type
                Some(pixels),
            )
        })
    }

    fn generate_mipmap(&self) {
        context::with(|ctx| ctx.generate_mipmap(Context::TEXTURE_2D))
    }

    // TODO
    fn active_unit(&self, unit_id: u32) {
        context::with(|ctx| {
            ctx.active_texture(Context::TEXTURE0);
        });
    }
}
