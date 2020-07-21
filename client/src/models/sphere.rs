use super::Model;
use crate::core::types::{Vec3, Vec4};
use cgmath::Vector3;
use palette::{Hsva, Srgba};

pub fn sphere(row: usize, column: usize, radius: f32) -> Model {
    let mut pos = Vec3::new();
    let mut nor = Vec3::new();
    let mut col = Vec4::new();
    let mut idx = Vec3::new();

    for i in 0..=row {
        let r = std::f32::consts::PI / row as f32 * i as f32;
        let ry = r.cos();
        let rr = r.sin();
        for ii in 0..=column {
            let tr = std::f32::consts::PI * 2.0 / column as f32 * ii as f32;
            let tx = rr * radius * tr.cos();
            let ty = ry * radius;
            let tz = rr * radius * tr.sin();
            let rx = rr * tr.cos();
            let rz = rr * tr.sin();
            pos.push_3(tx, ty, tz);
            nor.push_3(rx, ry, rz);

            let hsva = Hsva::new(360.0 / row as f32 * i as f32, 1.0, 1.0, 1.0);
            let rgba = Srgba::from(hsva);
            col.push_4(
                rgba.color.red,
                rgba.color.green,
                rgba.color.blue,
                rgba.alpha,
            );
        }
    }

    let (row, column) = (row as i16, column as i16);
    for i in 0..row {
        for ii in 0..column {
            let r = (column + 1) * i + ii;
            idx.push_3(r, r + 1, r + column + 2);
            idx.push_3(r, r + column + 2, r + column + 1);
        }
    }

    Model {
        positions: pos,
        colors: col,
        normals: nor,
        indexes: idx,
    }
}
