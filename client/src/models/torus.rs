use super::{Model, Vec3, Vec4};
use cgmath::{prelude::*, Rad};
use palette::{Hsva, Srgba};

const CIRCLE_RAD: Rad<f32> = Rad(std::f32::consts::PI * 2.0);

pub fn torus(tube_radius: f32, tube_smooth: u32, core_radius: f32, core_smooth: u32) -> Model {
    let mut positions = Vec3::new();
    let mut colors = Vec4::new();
    let mut indexes = Vec3::new();

    for tube_i in 0..=tube_smooth {
        let (tube_smooth_f, tube_i_f) = (tube_smooth as f32, tube_i as f32);

        let tube_rad = CIRCLE_RAD / tube_smooth_f * tube_i_f;
        let tube_x = tube_rad.cos() * tube_radius;
        let tube_y = tube_rad.sin() * tube_radius;

        for core_i in 0..=core_smooth {
            let (core_smooth_f, core_i_f) = (core_smooth as f32, core_i as f32);

            // 位置情報の計算
            let core_rad = CIRCLE_RAD / core_smooth_f * core_i_f;
            let x = (core_radius + tube_x) * core_rad.cos();
            let y = tube_y;
            let z = (core_radius + tube_x) * core_rad.sin();
            positions.push_3(x, y, z);

            // 色情報の計算
            let hsva = Hsva::new(360.0 / core_smooth_f * core_i_f, 1.0, 1.0, 1.0);
            let rgba = Srgba::from(hsva);
            colors.push_4(
                rgba.color.red,
                rgba.color.green,
                rgba.color.blue,
                rgba.alpha,
            );
        }
    }

    for tube_i in 0..tube_smooth {
        for core_i in 0..core_smooth {
            // index情報の計算
            let idx = ((core_smooth + 1) * core_i + tube_i) as i16;
            // 以下の4点で右に傾いてる平行四辺形を形成する
            let top_right = idx;
            let top_left = idx + 1;
            let bottom_right = idx + core_smooth as i16 + 1;
            let bottom_left = idx + core_smooth as i16 + 2;
            indexes.push_3(top_right, bottom_right, top_left);
            indexes.push_3(bottom_right, bottom_left, top_left);
        }
    }

    Model {
        positions,
        colors,
        indexes,
    }
}
