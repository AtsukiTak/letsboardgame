use crate::webgl::{
    buffers::{IBO, VBO},
    context::{self, Context},
    program::Program,
    shader::{
        Attribute, FragmentShader, Mat4, ParamsBase, ParamsVisitor, Uniform, Vec3, Vec4,
        VertexShader,
    },
};
use cgmath::{prelude::*, Deg, Matrix4, Point3, Vector3};
use wasm_bindgen::{prelude::*, JsCast as _};

pub async fn start() -> Result<(), JsValue> {
    initialize()?;

    let mut program = Program::<Params>::new(vert_shader()?, frag_shader()?)?;

    // 頂点VBOの生成
    let vertices_vbo = VBO::with_data(&[
        -0.5, 0.5, 0.0, // xyz
        0.5, 0.5, 0.0, // xyz
        -0.5, -0.5, 0.0, // xyz
        0.5, -0.5, 0.0, // xyz
    ]);
    program.params.position.attach_vbo(&vertices_vbo);

    // 色VBOの生成
    let colors_vbo = VBO::with_data(&[
        1.0, 0.0, 0.0, 1.0, // rgba
        0.0, 1.0, 0.0, 1.0, // rgba
        0.0, 0.0, 1.0, 1.0, // rgba
        1.0, 1.0, 1.0, 1.0, // rgba
    ]);
    program.params.color.attach_vbo(&colors_vbo);

    // IBOの生成
    let ibo = IBO::with_data(&[
        0, 2, 1, // 1つめの三角ポリゴン
        1, 2, 3, // 2つめの三角ポリゴン
    ]);
    ibo.bind();

    let mut frame = 0;
    loop {
        clear();
        render(&mut program, frame, vp_matrix());
        frame += 1;
        gloo_timers::future::TimeoutFuture::new(1000 / 60).await;
    }

    Ok(())
}

fn initialize() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    context::initialize(canvas)?;

    context::enable_culling();
    context::enable_depth_test();

    Ok(())
}

fn clear() {
    context::with(|ctx| {
        ctx.clear_color(0.0, 0.0, 0.0, 1.0);
        ctx.clear_depth(1.0);
        ctx.clear(Context::COLOR_BUFFER_BIT | Context::DEPTH_BUFFER_BIT);
    });
}

fn render(program: &mut Program<Params>, frame: usize, vp_matrix: Matrix4<f32>) {
    // web_sys::console::log_1(&format!("frame {}", frame).into());

    let mvp_matrix = vp_matrix * m_matrix(frame);
    program.params.mvp_matrix.set_value(mvp_matrix);
    context::with(|ctx| {
        ctx.draw_elements_with_i32(Context::TRIANGLES, 2 * 3, Context::UNSIGNED_SHORT, 0)
    });
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

fn vp_matrix() -> Matrix4<f32> {
    // ビュー座標変換行列
    let v_mat = Matrix4::look_at(
        Point3::new(0.0, 0.0, 3.0),  // カメラの位置
        Point3::new(0.0, 0.0, 0.0),  // 視点の中央
        Vector3::new(0.0, 1.0, 0.0), // 上方向のベクトル
    );

    // プロジェクション座標変換行列
    let p_mat = cgmath::perspective(
        Deg(100.0), // 画角
        1.0,        // アスペクト比
        0.1,        // どれくらい近くまでカメラに写すか
        100.0,      // どれくらい遠くまでカメラに写すか
    );

    p_mat * v_mat
}

fn m_matrix(frame: usize) -> Matrix4<f32> {
    let angle = Deg((frame * 6 % 360) as f32);
    Matrix4::from_translation(Vector3::new(angle.cos(), angle.sin(), 0.0))
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
