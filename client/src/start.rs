use crate::{
    context::{self, Context},
    shader::{
        Attribute, FragmentShader, Mat4, ParamsBase, ParamsVisitor, Program, Uniform, Vec3, Vec4,
        VertexShader,
    },
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

    let mut program = Program::<Params>::new(vert_shader()?, frag_shader()?)?;

    let vertices_vbo = VBO::with_data(&[
        -0.7, -0.7, 0.0, // xyz
        0.7, -0.7, 0.0, // xyz
        0.0, 0.7, 0.0, // xyz
    ]);
    program.params.position.attach_vbo(&vertices_vbo);

    let colors_vbo = VBO::with_data(&[
        1.0, 0.0, 0.0, 1.0, // rgba
        0.0, 1.0, 0.0, 1.0, // rgba
        0.0, 0.0, 1.0, 1.0, // rgba
    ]);
    program.params.color.attach_vbo(&colors_vbo);

    let (mvp_matrix1, mvp_matrix2) = mvp_matrix();

    program.params.mvp_matrix.set_value(mvp_matrix1);

    context::with(|ctx| ctx.draw_arrays(Context::TRIANGLES, 0, 3));

    program.params.mvp_matrix.set_value(mvp_matrix2);

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

    VertexShader::compile(src)
}

struct Params {
    position: Attribute<Vec3>,
    color: Attribute<Vec4>,
    mvp_matrix: Uniform<Mat4>,
}

impl ParamsBase for Params {
    fn from_visitor<'a>(visitor: ParamsVisitor<'a>) -> Result<Self, JsValue> {
        Ok(Params {
            position: visitor.visit_attr("position")?,
            color: visitor.visit_attr("color")?,
            mvp_matrix: visitor.visit_uniform("mvpMatrix")?,
        })
    }
}

fn mvp_matrix() -> (Matrix4<f32>, Matrix4<f32>) {
    // モデル座標変換行列
    let m_mat1 = Matrix4::from_translation(Vector3::new(-1.5, 0.0, 0.0));
    let m_mat2 = Matrix4::from_translation(Vector3::new(1.5, 0.0, 0.0));

    // ビュー座標変換行列
    let v_mat = Matrix4::look_at(
        Point3::new(0.0, 0.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    // プロジェクション座標変換行列
    let p_mat = cgmath::perspective(Deg(90.0), 1.0, 0.1, 100.0);

    (p_mat * v_mat * m_mat1, p_mat * v_mat * m_mat2)
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
