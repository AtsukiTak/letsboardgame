use super::{
    buffers::VBO,
    context::{self, Context},
    shader::{FragmentShader, VertexShader},
    types::{Mat4, Vec2, Vec3, Vec4},
};
use cgmath::{Matrix4, Vector3, Vector4};
use std::marker::PhantomData;
use wasm_bindgen::JsValue;

#[allow(dead_code)]
pub struct Program<P> {
    pub program: web_sys::WebGlProgram,
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

            let visitor = ParamsVisitor::new(ctx, &program);

            let params = P::from_visitor(visitor)?;

            Ok(Program {
                program,
                vert_shader,
                frag_shader,
                params,
            })
        })
    }

    // 現在のrenderingでこのprogramを使うことを宣言する
    pub fn use_program(&self) {
        context::with(|ctx| ctx.use_program(Some(&self.program)))
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

#[derive(Clone, Copy)]
pub struct ParamsVisitor<'a> {
    ctx: &'a Context,
    program: &'a web_sys::WebGlProgram,
}

impl<'a> ParamsVisitor<'a> {
    pub fn new(ctx: &'a Context, program: &'a web_sys::WebGlProgram) -> ParamsVisitor<'a> {
        ParamsVisitor { ctx, program }
    }

    pub fn visit_attr<T>(&self, name: &'static str) -> Result<Attribute<T>, JsValue> {
        // program中の位置（location）を取得
        let loc = self.ctx.get_attrib_location(self.program, name);

        if loc < 0 {
            let msg = format!("missing attribute \"{}\"", name);
            return Err(JsValue::from_str(msg.as_str()));
        }

        self.ctx.enable_vertex_attrib_array(loc as u32);

        Ok(Attribute::new(name, loc as u32))
    }

    pub fn visit_uniform<T>(&self, name: &'static str) -> Result<T, JsValue>
    where
        T: UniformBase,
    {
        if let Some(loc) = self.ctx.get_uniform_location(self.program, name) {
            Ok(T::from_parts(name, loc))
        } else {
            let msg = format!("missing uniform \"{}\"", name);
            Err(JsValue::from_str(msg.as_str()))
        }
    }
}

/*
 * ==========
 * Attribute
 * ==========
 */
pub struct Attribute<V> {
    #[allow(dead_code)]
    name: &'static str,
    location: u32,
    _value: PhantomData<V>,
}

impl<V> Attribute<V> {
    fn new(name: &'static str, location: u32) -> Self {
        Attribute {
            name,
            location,
            _value: PhantomData,
        }
    }
}

impl Attribute<Vec2<f32>> {
    pub fn attach_vbo(&self, vbo: &VBO<Vec2<f32>>) {
        vbo.bind();

        context::with(|ctx| {
            ctx.vertex_attrib_pointer_with_i32(self.location, 2, Context::FLOAT, false, 0, 0)
        });

        vbo.unbind();
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
    pub name: &'static str,
    pub location: web_sys::WebGlUniformLocation,
    pub value: V,
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

impl UniformBase for Uniform<Vector3<f32>> {
    fn from_parts(name: &'static str, location: web_sys::WebGlUniformLocation) -> Self {
        Uniform {
            name,
            location,
            value: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Uniform<Vector3<f32>> {
    pub fn set_value(&mut self, value: Vector3<f32>) {
        self.value = value;

        context::with(|ctx| {
            ctx.uniform3f(
                Some(&self.location),
                self.value.x,
                self.value.y,
                self.value.z,
            )
        })
    }
}

impl UniformBase for Uniform<Vector4<f32>> {
    fn from_parts(name: &'static str, location: web_sys::WebGlUniformLocation) -> Self {
        Uniform {
            name,
            location,
            value: Vector4::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

impl Uniform<Vector4<f32>> {
    pub fn set_value(&mut self, value: Vector4<f32>) {
        self.value = value;

        context::with(|ctx| {
            ctx.uniform4f(
                Some(&self.location),
                self.value.x,
                self.value.y,
                self.value.z,
                self.value.w,
            )
        })
    }
}

impl UniformBase for Uniform<i32> {
    fn from_parts(name: &'static str, location: web_sys::WebGlUniformLocation) -> Self {
        Uniform {
            name,
            location,
            value: 0,
        }
    }
}

impl Uniform<i32> {
    pub fn set_value(&mut self, value: i32) {
        self.value = value;

        context::with(|ctx| ctx.uniform1i(Some(&self.location), self.value))
    }
}
