use crate::webgl::{
    buffers::VBO,
    context::{self, Context},
    types::{Mat4, Vec3, Vec4},
};
use cgmath::Matrix4;
use std::marker::PhantomData;
use wasm_bindgen::JsValue;

pub struct VertexShader {
    pub shader: web_sys::WebGlShader,
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
    pub shader: web_sys::WebGlShader,
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
    pub fn new(ctx: &'a Context, program: &'a web_sys::WebGlProgram) -> ParamsVisitor<'a> {
        ParamsVisitor { ctx, program }
    }

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

impl AttributeBase for Attribute<Vec3<f32>> {
    fn from_parts(name: &'static str, location: u32) -> Self {
        Attribute {
            name,
            location,
            _value: PhantomData,
        }
    }
}

impl Attribute<Vec3<f32>> {
    pub fn attach_vbo(&self, vbo: &VBO<Vec3<f32>>) {
        vbo.bind();

        context::with(|ctx| {
            ctx.vertex_attrib_pointer_with_i32(self.location, 3, Context::FLOAT, false, 0, 0)
        });

        vbo.unbind();
    }
}

impl AttributeBase for Attribute<Vec4<f32>> {
    fn from_parts(name: &'static str, location: u32) -> Self {
        Attribute {
            name,
            location,
            _value: PhantomData,
        }
    }
}

impl Attribute<Vec4<f32>> {
    pub fn attach_vbo(&self, vbo: &VBO<Vec4<f32>>) {
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

impl UniformBase for Uniform<Mat4<f32>> {
    fn from_parts(name: &'static str, location: web_sys::WebGlUniformLocation) -> Self {
        Uniform {
            name,
            location,
            value: Mat4::default(),
        }
    }
}

impl Uniform<Mat4<f32>> {
    pub fn set_value(&mut self, value: Matrix4<f32>) {
        self.value = Mat4::new(value);

        context::with(|ctx| {
            ctx.uniform_matrix4fv_with_f32_array(Some(&self.location), false, self.value.as_ref())
        })
    }
}
