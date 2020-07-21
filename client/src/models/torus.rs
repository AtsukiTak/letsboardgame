use super::{Model, Vec3, Vec4};
use cgmath::{prelude::*, Rad, Vector3};
use palette::{Hsva, Srgba};

const CIRCLE_RAD: Rad<f32> = Rad(std::f32::consts::PI * 2.0);

pub fn torus(tube_radius: f32, tube_steps: u32, core_radius: f32, core_steps: u32) -> Model {
    let mut positions = Vec3::new();
    let mut colors = Vec4::new();
    let mut indexes = Vec3::new();
    let mut normals = Vec3::new();

    for tube_i in 0..=tube_steps {
        let (tube_steps_f, tube_i_f) = (tube_steps as f32, tube_i as f32);

        let tube_rad = CIRCLE_RAD / tube_steps_f * tube_i_f;
        let tube_x = tube_rad.cos() * tube_radius;
        let tube_y = tube_rad.sin() * tube_radius;

        for core_i in 0..=core_steps {
            let (core_steps_f, core_i_f) = (core_steps as f32, core_i as f32);

            // 位置情報の計算
            let core_rad = CIRCLE_RAD / core_steps_f * core_i_f;
            let x = (core_radius + tube_x) * core_rad.cos();
            let y = tube_y;
            let z = (core_radius + tube_x) * core_rad.sin();
            positions.push_3(x, y, z);

            // 色情報の計算
            let hsva = Hsva::new(360.0 / core_steps_f * core_i_f, 1.0, 1.0, 1.0);
            let rgba = Srgba::from(hsva);
            colors.push_4(
                rgba.color.red,
                rgba.color.green,
                rgba.color.blue,
                rgba.alpha,
            );

            // 法線情報の計算
            // let nx = tube_x * core_rad.cos();
            // let ny = tube_y;
            // let nz = tube_x * core_rad.sin();
            // だが、tube_x, tube_yはtube_radiusを共通に持つので
            // それを除いて計算する。
            // その場合、計算結果は0~1の範囲に収まる。
            let nx = tube_rad.cos() * core_rad.cos();
            let ny = tube_rad.sin();
            let nz = tube_rad.cos() * core_rad.sin();
            normals.push_3(nx, ny, nz);
        }
    }

    for tube_i in 0..tube_steps {
        for core_i in 0..core_steps {
            // index情報の計算
            let idx = ((core_steps + 1) * tube_i + core_i) as i16;
            // 以下の4点で右に傾いてる平行四辺形を形成する
            let top_right = idx;
            let top_left = idx + 1;
            let bottom_right = idx + core_steps as i16 + 1;
            let bottom_left = idx + core_steps as i16 + 2;
            indexes.push_3(top_right, bottom_right, top_left);
            indexes.push_3(bottom_right, bottom_left, top_left);
        }
    }

    Model {
        positions,
        colors,
        indexes,
        normals,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // テキスト（https://wgld.org/d/webgl/w021.html）に載っている
    // torus生成関数の単純な移植
    fn torus_origin_in_text(row: usize, column: usize, irad: f32, orad: f32) -> Model {
        let mut pos = Vec3::new();
        let mut nor = Vec3::new();
        let mut col = Vec4::new();
        let mut idx = Vec3::new();

        for i in 0..=row {
            let r = std::f32::consts::PI * 2.0 / row as f32 * i as f32;
            let rr = r.cos();
            let ry = r.sin();
            for ii in 0..=column {
                let tr = std::f32::consts::PI * 2.0 / column as f32 * ii as f32;
                let tx = (rr * irad + orad) * tr.cos();
                let ty = ry * irad;
                let tz = (rr * irad + orad) * tr.sin();
                let rx = rr * tr.cos();
                let rz = rr * tr.sin();
                pos.push_3(tx, ty, tz);
                nor.push_3(rx, ry, rz);
                let hsva = Hsva::new(360.0 / column as f32 * ii as f32, 1.0, 1.0, 1.0);
                let rgba = Srgba::from(hsva);
                col.push_4(
                    rgba.color.red,
                    rgba.color.green,
                    rgba.color.blue,
                    rgba.alpha,
                );
            }
        }

        for i in 0..(row as i16) {
            for ii in 0..(column as i16) {
                let r = (column as i16 + 1) * i + ii;
                idx.push_3(r, r + column as i16 + 1, r + 1);
                idx.push_3(r + column as i16 + 1, r + column as i16 + 2, r + 1);
            }
        }

        Model {
            positions: pos,
            colors: col,
            indexes: idx,
            normals: nor,
        }
    }

    #[test]
    fn test_torus() {
        let torus1 = torus(1.0, 64, 2.0, 64);
        let torus2 = torus_origin_in_text(64, 64, 1.0, 2.0);
        assert_eq!(torus1.positions, torus2.positions);
        assert_eq!(torus1.colors, torus2.colors);
        assert_eq!(torus1.indexes, torus2.indexes);
        assert_eq!(torus1.normals, torus2.normals);
    }
}
