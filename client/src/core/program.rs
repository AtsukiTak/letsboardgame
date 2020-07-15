use super::{
    context::{self, Context},
    shader::{FragmentShader, ParamsBase, ParamsVisitor, VertexShader},
};
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
}
