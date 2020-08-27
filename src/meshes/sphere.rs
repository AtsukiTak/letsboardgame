use super::Mesh;
use cgmath::{vec3, vec4, Vector3, Vector4};
use napier_webgl::vec::StepVec;
use palette::{Hsva, Srgba};

pub fn sphere(row: usize, column: usize, radius: f32) -> Mesh {
    let mut pos = StepVec::<Vector3<f32>>::new();
    let mut nor = StepVec::<Vector3<f32>>::new();
    let mut col = StepVec::<Vector4<f32>>::new();
    let mut idx = StepVec::<Vector3<i16>>::new();

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
            pos.push(vec3(tx, ty, tz));
            nor.push(vec3(rx, ry, rz));

            let hsva = Hsva::new(360.0 / row as f32 * i as f32, 1.0, 1.0, 1.0);
            let rgba = Srgba::from(hsva);
            col.push(vec4(
                rgba.color.red,
                rgba.color.green,
                rgba.color.blue,
                rgba.alpha,
            ));
        }
    }

    let (row, column) = (row as i16, column as i16);
    for i in 0..row {
        for ii in 0..column {
            let r = (column + 1) * i + ii;
            idx.push(vec3(r, r + 1, r + column + 2));
            idx.push(vec3(r, r + column + 2, r + column + 1));
        }
    }

    Mesh::new(pos, col, nor, idx)
}
