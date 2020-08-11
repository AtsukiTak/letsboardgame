use super::Mesh;
use crate::core::{
    color::Color,
    texture::Texture,
    types::{Vec2, Vec3, Vec4},
};

pub fn rect(width: f32, height: f32, color: Color) -> Mesh {
    // Zを書くように頂点を設定する
    let half_width = width / 2.0;
    let half_height = height / 2.0;
    let mut positions = Vec3::new();
    positions.push_3(-half_width, half_height, 0.0); // 左上
    positions.push_3(half_width, half_height, 0.0); // 右上
    positions.push_3(-half_width, -half_height, 0.0); // 左下
    positions.push_3(half_width, -half_height, 0.0); // 右下

    // 色は単色
    let (r, g, b, a) = color.to_f32();
    let mut colors = Vec4::new();
    colors.push_4(r, g, b, a);
    colors.push_4(r, g, b, a);
    colors.push_4(r, g, b, a);
    colors.push_4(r, g, b, a);

    // 法線はZ軸方向
    let mut normals = Vec3::new();
    normals.push_3(0.0, 0.0, 1.0);
    normals.push_3(0.0, 0.0, 1.0);
    normals.push_3(0.0, 0.0, 1.0);
    normals.push_3(0.0, 0.0, 1.0);

    let mut indexes = Vec3::new();
    indexes.push_3(0, 1, 2);
    indexes.push_3(1, 3, 2);

    Mesh::new(positions, colors, normals, indexes)
}

pub fn rect_with_texture(width: f32, height: f32, color: Color, texture: Texture) -> Mesh {
    let mut mesh = rect(width, height, color);

    // テクスチャ座標
    let mut tex_coord = Vec2::new();
    tex_coord.push_2(0.0, 0.0); // 左上
    tex_coord.push_2(1.0, 0.0); // 右上
    tex_coord.push_2(0.0, 1.0); // 左下
    tex_coord.push_2(1.0, 1.0); // 右下

    mesh.paste_texture(tex_coord, texture);

    mesh
}
