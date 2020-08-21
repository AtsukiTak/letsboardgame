use super::Mesh;
use crate::{Color, Texture};
use cgmath::{vec2, vec3, vec4, Vector2, Vector3, Vector4};
use napier_core::vec::StepVec;

pub fn rect(width: f32, height: f32, color: Color) -> Mesh {
    // Zを書くように頂点を設定する
    let half_width = width / 2.0;
    let half_height = height / 2.0;
    let mut positions = StepVec::<Vector3<f32>>::new();
    positions.push(vec3(-half_width, half_height, 0.0)); // 左上
    positions.push(vec3(half_width, half_height, 0.0)); // 右上
    positions.push(vec3(-half_width, -half_height, 0.0)); // 左下
    positions.push(vec3(half_width, -half_height, 0.0)); // 右下

    // 色は単色
    let (r, g, b, a) = color.to_f32();
    let mut colors = StepVec::<Vector4<f32>>::new();
    colors.push(vec4(r, g, b, a));
    colors.push(vec4(r, g, b, a));
    colors.push(vec4(r, g, b, a));
    colors.push(vec4(r, g, b, a));

    // 法線はZ軸方向
    let mut normals = StepVec::<Vector3<f32>>::new();
    normals.push(vec3(0.0, 0.0, 1.0));
    normals.push(vec3(0.0, 0.0, 1.0));
    normals.push(vec3(0.0, 0.0, 1.0));
    normals.push(vec3(0.0, 0.0, 1.0));

    let mut indexes = StepVec::<Vector3<i16>>::new();
    indexes.push(vec3(0, 2, 1));
    indexes.push(vec3(1, 2, 3));

    Mesh::new(positions, colors, normals, indexes)
}

pub fn rect_with_texture(width: f32, height: f32, color: Color, texture: Texture) -> Mesh {
    let mut mesh = rect(width, height, color);

    // テクスチャ座標
    let mut tex_coord = StepVec::<Vector2<f32>>::new();
    tex_coord.push(vec2(0.0, 0.0)); // 左上
    tex_coord.push(vec2(1.0, 0.0)); // 右上
    tex_coord.push(vec2(0.0, 1.0)); // 左下
    tex_coord.push(vec2(1.0, 1.0)); // 右下

    mesh.paste_texture(tex_coord, texture);

    mesh
}
