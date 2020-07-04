use wasm_bindgen::{prelude::*, JsCast as _};
use web_sys::WebGlRenderingContext as Context;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<Context>()?;

    let vert_shader = compile_shader(
        &context,
        Context::VERTEX_SHADER,
        r#"
        attribute vec4 position;
        void main() {
            gl_Position = position;
        }
        "#,
    )?;
    let frag_shader = compile_shader(
        &context,
        Context::FRAGMENT_SHADER,
        r#"
        void main() {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
        "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    // 頂点情報の用意.
    // 3要素（x, y, z) * 3頂点
    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    // bufferの作成
    // この時点ではまだVBOではない（VBO以外にも使えるbuffer)
    let buffer = context.create_buffer().unwrap();

    // ここでVBOとしてbufferをbind
    context.bind_buffer(Context::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            Context::ARRAY_BUFFER,
            &vert_array,
            Context::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(0, 3, Context::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(Context::COLOR_BUFFER_BIT);

    context.draw_arrays(Context::TRIANGLES, 0, (vertices.len() / 3) as i32);

    Ok(())
}

fn compile_shader(
    context: &Context,
    shader_type: u32,
    source: &str,
) -> Result<web_sys::WebGlShader, String> {
    // shaderの作成
    // shader_typeによって、頂点シェーダかフラグメントシェーダか決める
    let shader = context.create_shader(shader_type).unwrap();

    // shaderにソースコードを渡す
    context.shader_source(&shader, source);

    // shaderに渡したソースコードをコンパイル
    context.compile_shader(&shader);

    // コンパイルが成功したかどうか
    if context
        .get_shader_parameter(&shader, Context::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context.get_shader_info_log(&shader).unwrap())
    }
}

fn link_program(
    context: &Context,
    vert_shader: &web_sys::WebGlShader,
    frag_shader: &web_sys::WebGlShader,
) -> Result<web_sys::WebGlProgram, String> {
    let program = context.create_program().unwrap();

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    // programにattachした2つのshaderをlinkする
    context.link_program(&program);

    if context
        .get_program_parameter(&program, Context::LINK_STATUS)
        .as_bool()
        .unwrap()
    {
        Ok(program)
    } else {
        Err(context.get_program_info_log(&program).unwrap())
    }
}
