use crate::{
    context::{self, Context},
    vbo::VBO,
};
use cgmath::Matrix4;
use wasm_bindgen::JsValue;

pub struct Program {
    program: web_sys::WebGlProgram,
    vert_shader: VertexShader,
    frag_shader: FragmentShader,
}

impl Program {
    pub fn new(
        mut vert_shader: VertexShader,
        frag_shader: FragmentShader,
    ) -> Result<Program, JsValue> {
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

            // 各attributeを有効にする
            for attr in vert_shader.attrs.iter_mut() {
                // program中の位置（location）を取得
                let loc = ctx.get_attrib_location(&program, attr.name);
                if loc < 0 {
                    let msg = format!("missing vertex attribute \"{}\"", attr.name);
                    return Err(JsValue::from_str(msg.as_str()));
                }

                attr.index = Some(loc as u32);
                attr.enable();
            }

            // 各uniformのindexを取得する
            for uniform in vert_shader.uniforms.iter_mut() {
                if let Some(loc) = ctx.get_uniform_location(&program, uniform.name) {
                    uniform.location = Some(loc);
                    uniform.enable();
                } else {
                    let msg = format!("missing vertex uniform \"{}\"", uniform.name);
                    return Err(JsValue::from_str(msg.as_str()));
                }
            }

            Ok(Program {
                program,
                vert_shader,
                frag_shader,
            })
        })
    }

    pub fn vert_attr(&self, attr_name: &str) -> Option<&Attribute> {
        self.vert_shader
            .attrs
            .iter()
            .find(|attr| attr.name == attr_name)
    }
}

/*
 * ===========
 * Shader
 * ===========
 */
pub struct VertexShader {
    shader: web_sys::WebGlShader,
    attrs: Vec<Attribute>,
    uniforms: Vec<Uniform>,
}

pub struct FragmentShader {
    shader: web_sys::WebGlShader,
}

impl VertexShader {
    /// # NOTE
    /// Panic if context is uninitialized.
    pub fn compile(
        src: &str,
        attrs: Vec<Attribute>,
        uniforms: Vec<Uniform>,
    ) -> Result<Self, JsValue> {
        context::with(|ctx| {
            let shader = compile(ctx, src, Context::VERTEX_SHADER)?;
            Ok(VertexShader {
                shader,
                attrs,
                uniforms,
            })
        })
    }
}

impl FragmentShader {
    /// # NOTE
    /// Panic if context is uninitialized.
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
 * ==========
 * Attribute
 * ==========
 */
pub struct Attribute {
    pub name: &'static str,
    pub type_: AttributeType,
    pub index: Option<u32>,
}

impl Attribute {
    pub fn new(name: &'static str, type_: AttributeType) -> Self {
        Attribute {
            name,
            type_,
            index: None,
        }
    }

    pub fn enable(&self) {
        context::with(|ctx| ctx.enable_vertex_attrib_array(self.index.unwrap()))
    }

    pub fn attach_vbo(&self, vbo: &VBO) {
        let index = self
            .index
            .expect("cannot attach vbo to vertex attribute unlinked with any program");

        vbo.bind();

        context::with(|ctx| {
            ctx.vertex_attrib_pointer_with_i32(
                index,
                self.type_.stride() as i32,
                Context::FLOAT,
                false,
                0,
                0,
            )
        });

        vbo.unbind();
    }
}

pub enum AttributeType {
    Vec3,
    Vec4,
}

impl AttributeType {
    pub fn stride(&self) -> u32 {
        match self {
            AttributeType::Vec3 => 3,
            AttributeType::Vec4 => 4,
        }
    }
}

/*
 * ==========
 * Uniform
 * ==========
 */
pub struct Uniform {
    name: &'static str,
    value: UniformVal,
    location: Option<web_sys::WebGlUniformLocation>,
}

pub enum UniformVal {
    /* TODO
    Float2,
    Float3,
    Float4,
    Int2,
    Int3,
    Int4,
    VecFloat2,
    VecFloat3,
    VecFloat4,
    VecInt2,
    VecInt3,
    VecInt4,
    Matrix2,
    Matrix3,
    */
    Matrix4(Matrix4<f32>),
}

impl Uniform {
    pub fn new_mat4(name: &'static str, value: Matrix4<f32>) -> Self {
        Uniform {
            name,
            value: UniformVal::Matrix4(value),
            location: None,
        }
    }

    pub fn enable(&self) {
        context::with(|ctx| match &self.value {
            UniformVal::Matrix4(val) => ctx.uniform_matrix4fv_with_f32_array(
                Some(self.location.as_ref().unwrap()),
                false,
                AsRef::<[f32; 16]>::as_ref(val).as_ref(),
            ),
        })
    }
}
