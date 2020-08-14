use super::{
    buffers::VBO,
    context::{self, Context},
    shader::{FragmentShader, VertexShader},
    texture::GlTextureUnit,
    vec::StepVec,
};
use cgmath::{Array, Matrix4, Vector3, Vector4};
use std::marker::PhantomData;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;

#[allow(dead_code)]
pub struct GlProgram<P> {
    pub(crate) program: web_sys::WebGlProgram,
    vert_shader: VertexShader,
    frag_shader: FragmentShader,
    pub(crate) params: P,
    vertex_attrib_locations: Vec<u32>,
}

impl<P> GlProgram<P>
where
    P: ParamsBase,
{
    pub fn new(
        vert_shader: VertexShader,
        frag_shader: FragmentShader,
    ) -> Result<GlProgram<P>, JsValue> {
        context::with(|ctx| {
            let program = ctx.create_program().unwrap();

            // 作成したprogramを各shaderを関連づける
            ctx.attach_shader(&program, &vert_shader.shader);
            ctx.attach_shader(&program, &frag_shader.shader);

            // contextにprogramをlinkする (両shaderをlinkする)
            // 両shaderに対するGPUコードの準備を完了させる
            ctx.link_program(&program);

            let success_link = ctx
                .get_program_parameter(&program, GL::LINK_STATUS)
                .as_bool()
                .unwrap();

            if !success_link {
                let err_msg = ctx.get_program_info_log(&program).unwrap();
                return Err(JsValue::from_str(err_msg.as_str()));
            }

            let mut visitor = ParamsVisitor::new(ctx, &program);

            let params = P::from_visitor(&mut visitor)?;

            let vertex_attrib_locations = visitor.vertex_attrib_locations;

            Ok(GlProgram {
                program,
                vert_shader,
                frag_shader,
                params,
                vertex_attrib_locations,
            })
        })
    }
}

impl<P> GlProgram<P> {
    pub fn vertex_attrib_locations(&self) -> &[u32] {
        self.vertex_attrib_locations.as_ref()
    }
}

/*
 * ========
 * Params
 * ========
 */
pub trait ParamsBase {
    fn from_visitor<'a>(visitor: &mut ParamsVisitor<'a>) -> Result<Self, JsValue>
    where
        Self: Sized;
}

pub struct ParamsVisitor<'a> {
    ctx: &'a Context,
    program: &'a web_sys::WebGlProgram,
    vertex_attrib_locations: Vec<u32>,
}

impl<'a> ParamsVisitor<'a> {
    pub fn new(ctx: &'a Context, program: &'a web_sys::WebGlProgram) -> ParamsVisitor<'a> {
        ParamsVisitor {
            ctx,
            program,
            vertex_attrib_locations: Vec::new(),
        }
    }

    pub fn visit_attr<A>(&mut self, name: &'static str) -> Result<Attribute<StepVec<A>>, JsValue>
    where
        A: Array<Element = f32>,
    {
        // program中の位置（location）を取得
        let loc = self.ctx.get_attrib_location(self.program, name);

        if loc < 0 {
            let msg = format!("missing attribute \"{}\"", name);
            return Err(JsValue::from_str(msg.as_str()));
        }
        let loc = loc as u32;

        self.vertex_attrib_locations.push(loc);

        Ok(Attribute::new(name, loc))
    }

    pub fn visit_uniform<T>(&self, name: &'static str) -> Result<Uniform<T>, JsValue> {
        if let Some(loc) = self.ctx.get_uniform_location(self.program, name) {
            Ok(Uniform::new(name, loc))
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

impl<A> Attribute<StepVec<A>>
where
    A: Array<Element = f32>,
{
    fn new(name: &'static str, location: u32) -> Self {
        Attribute {
            name,
            location,
            _value: PhantomData,
        }
    }

    pub fn attach_vbo(&self, vbo: &VBO<StepVec<A>>) {
        vbo.bind();

        context::with(|ctx| {
            ctx.vertex_attrib_pointer_with_i32(
                self.location,
                StepVec::<A>::step() as i32,
                GL::FLOAT,
                false,
                0,
                0,
            )
        });

        vbo.unbind();
    }
}

/*
 * ==========
 * Uniform
 * ==========
 */
pub struct Uniform<V> {
    #[allow(dead_code)]
    pub name: &'static str,
    pub location: web_sys::WebGlUniformLocation,
    pub value: Option<V>,
}

impl<V> Uniform<V> {
    fn new(name: &'static str, location: web_sys::WebGlUniformLocation) -> Self {
        Uniform {
            name,
            location,
            value: None,
        }
    }
}

impl Uniform<Matrix4<f32>> {
    pub fn set_value(&mut self, value: Matrix4<f32>) {
        context::with(|ctx| {
            ctx.uniform_matrix4fv_with_f32_array(
                Some(&self.location),
                false,
                &AsRef::<[f32; 16]>::as_ref(&value)[..],
            )
        });

        self.value = Some(value);
    }
}

impl Uniform<Vector3<f32>> {
    pub fn set_value(&mut self, value: Vector3<f32>) {
        context::with(|ctx| ctx.uniform3f(Some(&self.location), value.x, value.y, value.z));

        self.value = Some(value);
    }
}

impl Uniform<Vector4<f32>> {
    pub fn set_value(&mut self, value: Vector4<f32>) {
        context::with(|ctx| {
            ctx.uniform4f(Some(&self.location), value.x, value.y, value.z, value.w)
        });

        self.value = Some(value);
    }
}

impl Uniform<i32> {
    pub fn set_value(&mut self, value: i32) {
        context::with(|ctx| ctx.uniform1i(Some(&self.location), value));
        self.value = Some(value);
    }
}

impl Uniform<GlTextureUnit> {
    pub fn set_value(&mut self, value: GlTextureUnit) {
        let i = value.to_int();
        context::with(|ctx| ctx.uniform1i(Some(&self.location), i));
        self.value = Some(value);
    }
}
