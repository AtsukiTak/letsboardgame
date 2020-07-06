use cgmath::{perspective, prelude::*, Deg, Matrix4, Point3, Vector3};
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

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear_depth(1.0);
    context.clear(Context::COLOR_BUFFER_BIT | Context::DEPTH_BUFFER_BIT);

    let vert_shader = compile_shader(
        &context,
        Context::VERTEX_SHADER,
        r#"
        attribute   vec3 position;
        attribute   vec4 color;
        uniform     mat4 mvpMatrix;
        varying     vec4 vColor;

        void main() {
            vColor = color;
            gl_Position = mvpMatrix * vec4(position, 1.0);
        }
        "#,
    )?;
    let frag_shader = compile_shader(
        &context,
        Context::FRAGMENT_SHADER,
        r#"
        precision   mediump float;
        varying     vec4    vColor;

        void main() {
            gl_FragColor = vColor;
        }
        "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;

    // 各 attributeが何番目のattributeか
    let pos_attr_loc = context.get_attrib_location(&program, "position") as u32;
    let color_attr_loc = context.get_attrib_location(&program, "color") as u32;

    // "position" attribute を有効にする
    context.enable_vertex_attrib_array(color_attr_loc);
    context.enable_vertex_attrib_array(pos_attr_loc);

    // "position" attributeは3つの要素(x, y, z)で1つのデータを表す (vec3型)
    let pos_attr_stride = 3;
    // "color" attributeは4つの要素(RGBA)で1つのデータを表す (vec4型)
    let color_attr_stride = 4;

    // 頂点情報の用意.
    // 3要素 (x, y, z) * 3頂点
    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    // 色情報の用意
    // 4要素 (RGBA) * 4頂点
    let colors: [f32; 12] = [1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0];

    // VBOの作成とbind
    let vert_vbo = create_vbo(&context, &vertices);
    let color_vbo = create_vbo(&context, &colors);

    // 以降の操作のために、"position" attribute のvboをbind
    context.bind_buffer(Context::ARRAY_BUFFER, Some(&vert_vbo));

    // shaderにデータを登録
    // 対象となるVBOを必ずbindしておく
    // じゃないと、どのVBOを対象のattributeに関連付けるかが分からない
    context.vertex_attrib_pointer_with_i32(
        pos_attr_loc,
        pos_attr_stride,
        Context::FLOAT,
        false,
        0,
        0,
    );

    // 以降の操作のために、"color" attributeのvboをbind
    context.bind_buffer(Context::ARRAY_BUFFER, Some(&color_vbo));
    context.vertex_attrib_pointer_with_i32(
        color_attr_loc,
        color_attr_stride,
        Context::FLOAT,
        false,
        0,
        0,
    );

    let mvp_mat = create_mvp_matrix();
    let mvp_mat_array: &[f32; 16] = mvp_mat.as_ref();
    let uni_loc = context.get_uniform_location(&program, "mvpMatrix").unwrap();
    context.uniform_matrix4fv_with_f32_array(Some(&uni_loc), false, mvp_mat_array.as_ref());

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
        context.use_program(Some(&program));
        Ok(program)
    } else {
        Err(context.get_program_info_log(&program).unwrap())
    }
}

fn create_vbo(context: &Context, vertices: &[f32]) -> web_sys::WebGlBuffer {
    // bufferの作成
    // この時点ではまだVBOではない（VBO以外にも使えるbuffer)
    let buffer = context.create_buffer().unwrap();

    // ここでVBOとしてbufferをbind
    context.bind_buffer(Context::ARRAY_BUFFER, Some(&buffer));

    let vert_array = js_sys::Float32Array::from(vertices);

    // bufferにデータをセット
    context.buffer_data_with_array_buffer_view(
        Context::ARRAY_BUFFER,
        &vert_array,
        Context::STATIC_DRAW,
    );

    // bufferをunbind
    context.bind_buffer(Context::ARRAY_BUFFER, None);

    buffer
}

fn create_mvp_matrix() -> Matrix4<f32> {
    // モデル座標変換行列
    let m_mat = Matrix4::identity();

    // ビュー座標変換行列
    let v_mat = Matrix4::look_at(
        Point3::new(0.0, 1.0, 3.0),
        Point3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    // プロジェクション座標変換行列
    let p_mat = perspective(Deg(90.0), 1.0, 0.1, 100.0);

    p_mat * v_mat * m_mat
}
