use super::context;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;

#[derive(Debug, PartialEq)]
pub struct GlTexture {
    gl_texture: web_sys::WebGlTexture,
}

impl GlTexture {
    pub fn new() -> GlTexture {
        context::with(|ctx| GlTexture {
            gl_texture: ctx.create_texture().unwrap(),
        })
    }

    /// 画像データとともにGlTextureオブジェクトを初期化する
    /// 画像サイズは、縦横それぞれ2の冪乗でなければならない
    pub fn with_raw_image(pixels: &[u8], width: i32, height: i32) -> Result<GlTexture, JsValue> {
        assert_eq!(width.count_ones(), 1);
        assert_eq!(height.count_ones(), 1);

        let tex = GlTexture::new();
        tex.bind();
        tex.attach_img(pixels, width, height)?;
        tex.generate_mipmap();
        tex.unbind();
        Ok(tex)
    }

    // テクスチャユニットを有効化し、そこにテクスチャをbindする
    pub(crate) fn attach_unit(&self, unit_id: u32) {
        self.activate_unit(unit_id);
        self.bind();
    }

    fn bind(&self) {
        context::with(|ctx| ctx.bind_texture(GL::TEXTURE_2D, Some(&self.gl_texture)))
    }

    fn unbind(&self) {
        context::with(|ctx| ctx.bind_texture(GL::TEXTURE_2D, None))
    }

    fn attach_img(&self, pixels: &[u8], width: i32, height: i32) -> Result<(), JsValue> {
        context::with(|ctx| {
            ctx.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                GL::TEXTURE_2D,  // target
                0,               // level
                GL::RGBA as i32, // internal format
                width,
                height,
                0,                 // border. Must be 0.
                GL::RGBA,          // format
                GL::UNSIGNED_BYTE, // type
                Some(pixels),
            )
        })
    }

    fn generate_mipmap(&self) {
        context::with(|ctx| ctx.generate_mipmap(GL::TEXTURE_2D))
    }

    fn activate_unit(&self, unit_id: u32) {
        let unit = match unit_id {
            0 => GL::TEXTURE0,
            1 => GL::TEXTURE1,
            2 => GL::TEXTURE2,
            3 => GL::TEXTURE3,
            4 => GL::TEXTURE4,
            5 => GL::TEXTURE5,
            6 => GL::TEXTURE6,
            7 => GL::TEXTURE7,
            _ => panic!("Supported unit_id is 0 ~ 7."),
        };

        context::with(|ctx| {
            ctx.active_texture(unit);
        });
    }
}
