use super::context::{self, Context};
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;

pub struct VertexShader {
    pub shader: web_sys::WebGlShader,
}

impl VertexShader {
    pub fn compile(src: &str) -> Result<Self, JsValue> {
        context::with(|ctx| {
            let shader = compile(ctx, src, GL::VERTEX_SHADER)?;
            Ok(VertexShader { shader })
        })
    }
}

pub struct FragmentShader {
    pub shader: web_sys::WebGlShader,
}

impl FragmentShader {
    pub fn compile(src: &str) -> Result<Self, JsValue> {
        context::with(|ctx| {
            let shader = compile(ctx, src, GL::FRAGMENT_SHADER)?;
            Ok(FragmentShader { shader })
        })
    }
}

fn compile(ctx: &Context, src: &str, shader_type: u32) -> Result<web_sys::WebGlShader, JsValue> {
    // shaderオブジェクトの作成
    let shader = ctx.create_shader(shader_type).unwrap();

    // shaderにソースコードを渡す
    ctx.shader_source(&shader, src);

    // コンパイル
    ctx.compile_shader(&shader);

    // コンパイルに成功したかどうか
    if ctx
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let err_msg = ctx.get_shader_info_log(&shader).unwrap();
        Err(JsValue::from_str(err_msg.as_str()))
    }
}
