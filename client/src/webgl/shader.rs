use crate::webgl::{
    context::{self, Context},
    vbo::VBO,
};
use cgmath::{prelude::*, Matrix4};
use std::marker::PhantomData;
use wasm_bindgen::JsValue;

#[allow(dead_code)]
pub struct Program<P> {
    program: web_sys::WebGlProgram,
    vert_shader: VertexShader,
    frag_shader: FragmentShader,
    pub params: P,
}

impl<P> Program<P>
where
    P: ParamsBase,
{
    pub fn new(
        vert_shader: VertexShader,
        frag_shader: FragmentShader,
    ) -> Result<Program<P>, JsValue> {
        context::with(|ctx| {
            let program = ctx.create_program().unwrap();

            // 作成したprogramを各shaderを関連づける
            ctx.attach_shader(&program, &vert_shader.shader);
            ctx.attach_shader(&program, &frag_shader.shader);

            // contextにprogramをlinkする (両shaderをlinkする)
            // 両shaderに対するGPUコードの準備を完了させる
            ctx.link_program(&program);

            let success_link = ctx
                .get_program_parameter(&program, Context::LINK_STATUS)
                .as_bool()
                .unwrap();

            if !success_link {
                let err_msg = ctx.get_program_info_log(&program).unwrap();
                return Err(JsValue::from_str(err_msg.as_str()));
            }

            // 現在のrenderingでこのprogramを使うことを宣言する
            ctx.use_program(Some(&program));

            let visitor = ParamsVisitor {
                ctx,
                program: &program,
            };

            let params = P::from_visitor(visitor)?;

            Ok(Program {
                program,
                vert_shader,
                frag_shader,
                params,
            })
        })
    }
}

/*
 * ===========
 * Shader
 * ===========
 */
pub struct VertexShader {
    shader: web_sys::WebGlShader,
}

impl VertexShader {
    pub fn compile(src: &str) -> Result<Self, JsValue> {
        context::with(|ctx| {
            let shader = compile(ctx, src, Context::VERTEX_SHADER)?;
            Ok(VertexShader { shader })
        })
    }
}

pub struct FragmentShader {
    shader: web_sys::WebGlShader,
}

impl FragmentShader {
    pub fn compile(src: &str) -> Result<Self, JsValue> {
        context::with(|ctx| {
            let shader = compile(ctx, src, Context::FRAGMENT_SHADER)?;
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
        .get_shader_parameter(&shader, Context::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let err_msg = ctx.get_shader_info_log(&shader).unwrap();
        Err(JsValue::from_str(err_msg.as_str()))
    }
}

/*
 * ========
 * Params
 * ========
 */
pub trait ParamsBase {
    fn from_visitor<'a>(visitor: ParamsVisitor<'a>) -> Result<Self, JsValue>
    where
        Self: Sized;
}

pub struct ParamsVisitor<'a> {
    ctx: &'a Context,
    program: &'a web_sys::WebGlProgram,
}

impl<'a> ParamsVisitor<'a> {
    pub fn visit_attr<T>(&self, name: &'static str) -> Result<T, JsValue>
    where
        T: AttributeBase,
    {
        // program中の位置（location）を取得
        let loc = self.ctx.get_attrib_location(self.program, name);

        if loc < 0 {
            let msg = format!("missing vertex attribute \"{}\"", name);
            return Err(JsValue::from_str(msg.as_str()));
        }

        self.ctx.enable_vertex_attrib_array(loc as u32);

        Ok(T::from_parts(name, loc as u32))
    }

    pub fn visit_uniform<T>(&self, name: &'static str) -> Result<T, JsValue>
    where
        T: UniformBase,
    {
        if let Some(loc) = self.ctx.get_uniform_location(self.program, name) {
            Ok(T::from_parts(name, loc))
        } else {
            let msg = format!("missing vertex uniform \"{}\"", name);
            Err(JsValue::from_str(msg.as_str()))
        }
    }
}

#[derive(Default)]
pub struct Vec3(Vec<f32>);

#[derive(Default)]
pub struct Vec4(Vec<f32>);

pub struct Mat4(Matrix4<f32>);

impl AsRef<[f32]> for Mat4 {
    fn as_ref(&self) -> &[f32] {
        AsRef::<[f32; 16]>::as_ref(&self.0).as_ref()
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Mat4(Matrix4::identity())
    }
}

impl From<Matrix4<f32>> for Mat4 {
    fn from(value: Matrix4<f32>) -> Self {
        Mat4(value)
    }
}

/*
 * ==========
 * Attribute
 * ==========
 */
pub trait AttributeBase {
    fn from_parts(name: &'static str, location: u32) -> Self;
}

pub struct Attribute<V> {
    #[allow(dead_code)]
    name: &'static str,
    location: u32,
    _value: PhantomData<V>,
}

impl AttributeBase for Attribute<Vec3> {
    fn from_parts(name: &'static str, location: u32) -> Self {
        Attribute {
            name,
            location,
            _value: PhantomData,
        }
    }
}

impl Attribute<Vec3> {
    pub fn attach_vbo(&self, vbo: &VBO) {
        vbo.bind();

        context::with(|ctx| {
            ctx.vertex_attrib_pointer_with_i32(self.location, 3, Context::FLOAT, false, 0, 0)
        });

        vbo.unbind();
    }
}

impl AttributeBase for Attribute<Vec4> {
    fn from_parts(name: &'static str, location: u32) -> Self {
        Attribute {
            name,
            location,
            _value: PhantomData,
        }
    }
}

impl Attribute<Vec4> {
    pub fn attach_vbo(&self, vbo: &VBO) {
        vbo.bind();

        context::with(|ctx| {
            ctx.vertex_attrib_pointer_with_i32(self.location, 4, Context::FLOAT, false, 0, 0)
        });

        vbo.unbind();
    }
}

/*
 * ==========
 * Uniform
 * ==========
 */
pub trait UniformBase {
    fn from_parts(name: &'static str, location: web_sys::WebGlUniformLocation) -> Self;
}

pub struct Uniform<V> {
    #[allow(dead_code)]
    name: &'static str,
    location: web_sys::WebGlUniformLocation,
    value: V,
}

impl UniformBase for Uniform<Mat4> {
    fn from_parts(name: &'static str, location: web_sys::WebGlUniformLocation) -> Self {
        Uniform {
            name,
            location,
            value: Mat4::default(),
        }
    }
}

impl Uniform<Mat4> {
    pub fn set_value(&mut self, value: Matrix4<f32>) {
        self.value = Mat4(value);

        context::with(|ctx| {
            ctx.uniform_matrix4fv_with_f32_array(Some(&self.location), false, self.value.as_ref())
        })
    }
}
