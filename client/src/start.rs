use crate::{
    context::{self, Context},
    shader::{Attribute, AttributeType, FragmentShader, Program, Uniform, VertexShader},
    vbo::VBO,
};
use cgmath::{prelude::*, Deg, Matrix4, Point3, Vector3};
use wasm_bindgen::{prelude::*, JsCast as _};

pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    context::initialize(canvas)?;

    context::with(|ctx| {
        ctx.clear_color(0.0, 0.0, 0.0, 1.0);
        ctx.clear_depth(1.0);
        ctx.clear(Context::COLOR_BUFFER_BIT | Context::DEPTH_BUFFER_BIT);
    });

    let program = Program::new(vert_shader()?, frag_shader()?)?;

    let vertices_vbo = VBO::with_data(&[
        -0.7, -0.7, 0.0, // xyz
        0.7, -0.7, 0.0, // xyz
        0.0, 0.7, 0.0, // xyz
    ]);
    program
        .vert_attr("position")
        .unwrap()
        .attach_vbo(&vertices_vbo);

    let colors_vbo = VBO::with_data(&[
        1.0, 0.0, 0.0, 1.0, // rgba
        0.0, 1.0, 0.0, 1.0, // rgba
        0.0, 0.0, 1.0, 1.0, // rgba
    ]);
    program.vert_attr("color").unwrap().attach_vbo(&colors_vbo);

    context::with(|ctx| ctx.draw_arrays(Context::TRIANGLES, 0, 3));

    Ok(())
}

fn vert_shader() -> Result<VertexShader, JsValue> {
    let src = r#"
        attribute   vec3 position;
        attribute   vec4 color;
        uniform     mat4 mvpMatrix;
        varying     vec4 vColor;

        void main() {
            vColor = color;
            gl_Position = mvpMatrix * vec4(position, 1.0);
        }
    "#;

    let attrs = vec![
        Attribute::new("position", AttributeType::Vec3),
        Attribute::new("color", AttributeType::Vec4),
    ];

    let uniforms = vec![Uniform::new_mat4("mvpMatrix", mvp_matrix())];

    VertexShader::compile(src, attrs, uniforms)
}

fn mvp_matrix() -> Matrix4<f32> {
    // モデル座標変換行列
    let m_mat = Matrix4::identity();

    // ビュー座標変換行列
    let v_mat = Matrix4::look_at(
        Point3::new(0.0, 1.0, 3.0),
        Point3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    // プロジェクション座標変換行列
    let p_mat = cgmath::perspective(Deg(90.0), 1.0, 0.1, 100.0);

    p_mat * v_mat * m_mat
}

fn frag_shader() -> Result<FragmentShader, JsValue> {
    let src = r#"
        precision   mediump float;
        varying     vec4    vColor;

        void main() {
            gl_FragColor = vColor;
        }
    "#;

    FragmentShader::compile(src)
}
