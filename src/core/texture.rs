use super::context;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;

#[derive(Debug, PartialEq)]
pub struct GlTexture {
    gl_texture: web_sys::WebGlTexture,
}

impl GlTexture {
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

    pub fn bind(&self) {
        context::with(|ctx| ctx.bind_texture(GL::TEXTURE_2D, Some(&self.gl_texture)))
    }

    pub fn unbind(&self) {
        context::with(|ctx| ctx.bind_texture(GL::TEXTURE_2D, None))
    }

    /// 縮小表示するときの補完方法を指定する
    /// この関数を呼び出す前に、bindを実行しておく必要がある
    pub fn set_minify_filter(&self, method: MinMethod) {
        context::with(|ctx| {
            ctx.tex_parameteri(
                GL::TEXTURE_2D,
                GL::TEXTURE_MIN_FILTER,
                method.to_gl() as i32,
            )
        })
    }

    /// 拡大表示するときの保管方法を指定する
    /// この関数を呼び出す前に、bindを実行しておく必要がある
    pub fn set_magnify_filter(&self, method: MagMethod) {
        context::with(|ctx| {
            ctx.tex_parameteri(
                GL::TEXTURE_2D,
                GL::TEXTURE_MAG_FILTER,
                method.to_gl() as i32,
            )
        })
    }

    fn new() -> GlTexture {
        context::with(|ctx| GlTexture {
            gl_texture: ctx.create_texture().unwrap(),
        })
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
}

#[allow(dead_code)]
pub enum GlTextureUnit {
    Unit0,
    Unit1,
    Unit2,
    Unit3,
    Unit4,
    Unit5,
    Unit6,
    Unit7,
}

impl GlTextureUnit {
    pub fn activate(&self) {
        use GlTextureUnit::*;

        let unit = match self {
            Unit0 => GL::TEXTURE0,
            Unit1 => GL::TEXTURE1,
            Unit2 => GL::TEXTURE2,
            Unit3 => GL::TEXTURE3,
            Unit4 => GL::TEXTURE4,
            Unit5 => GL::TEXTURE5,
            Unit6 => GL::TEXTURE6,
            Unit7 => GL::TEXTURE7,
        };

        context::with(|ctx| {
            ctx.active_texture(unit);
        });
    }

    pub fn to_int(&self) -> i32 {
        use GlTextureUnit::*;

        match self {
            Unit0 => 0,
            Unit1 => 1,
            Unit2 => 2,
            Unit3 => 3,
            Unit4 => 4,
            Unit5 => 5,
            Unit6 => 6,
            Unit7 => 7,
        }
    }
}

/// 縮小表示するときの補完方法
/// 下に行くほど、高品質、高負荷になる。
#[allow(dead_code)]
pub enum MinMethod {
    /// 対象ピクセルの中心に最も近い点の値をそのまま採用
    Nearest,
    /// 対象ピクセルに最も近い4点の値を加重平均化して採用
    Linear,
    /// 最適なミップマップを選択
    /// さらに gl.NEAREST 準拠で値を採用
    NearestMipmapNearest,
    /// 最適なミップマップを選択
    /// さらに gl.LINEAR 準拠で値を採用
    NearestMipmapLinear,
    /// 最適なミップマップを二つ選択
    /// さらに gl.NEAREST 準拠でそれぞれ値を取り
    /// それらの値の加重平均を最終的に採用
    LinearMipmapNearest,
    /// 最適なミップマップを二つ選択
    /// さらに gl.NEAREST 準拠でそれぞれ値を取り
    /// それらの値の加重平均を最終的に採用
    LinearMipmapLinear,
}

impl MinMethod {
    pub fn to_gl(&self) -> u32 {
        use MinMethod::*;
        match self {
            Nearest => GL::NEAREST,
            Linear => GL::LINEAR,
            NearestMipmapNearest => GL::NEAREST_MIPMAP_NEAREST,
            NearestMipmapLinear => GL::NEAREST_MIPMAP_LINEAR,
            LinearMipmapNearest => GL::LINEAR_MIPMAP_NEAREST,
            LinearMipmapLinear => GL::LINEAR_MIPMAP_LINEAR,
        }
    }
}

/// 拡大表示するときの補完方法
/// 下に行くほど、高品質、高負荷になる。
#[allow(dead_code)]
pub enum MagMethod {
    /// 対象ピクセルの中心に最も近い点の値をそのまま採用
    Nearest,
    /// 対象ピクセルに最も近い4点の値を加重平均化して採用
    Linear,
}

impl MagMethod {
    pub fn to_gl(&self) -> u32 {
        use MagMethod::*;
        match self {
            Nearest => GL::NEAREST,
            Linear => GL::LINEAR,
        }
    }
}
