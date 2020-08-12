use super::Mesh;
use crate::core::types::StepVec;
use cgmath::{prelude::*, vec3, vec4, Rad, Vector3, Vector4};
use palette::{Hsva, Srgba};

const CIRCLE_RAD: Rad<f32> = Rad(std::f32::consts::PI * 2.0);

pub fn torus(tube_radius: f32, tube_steps: u32, core_radius: f32, core_steps: u32) -> Mesh {
    let mut positions = StepVec::<Vector3<f32>>::new();
    let mut colors = StepVec::<Vector4<f32>>::new();
    let mut indexes = StepVec::<Vector3<i16>>::new();
    let mut normals = StepVec::<Vector3<f32>>::new();

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
            positions.push(vec3(x, y, z));

            // 色情報の計算
            let hsva = Hsva::new(360.0 / core_steps_f * core_i_f, 1.0, 1.0, 1.0);
            let rgba = Srgba::from(hsva);
            colors.push(vec4(
                rgba.color.red,
                rgba.color.green,
                rgba.color.blue,
                rgba.alpha,
            ));

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
            normals.push(vec3(nx, ny, nz));
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
            indexes.push(vec3(top_right, bottom_right, top_left));
            indexes.push(vec3(bottom_right, bottom_left, top_left));
        }
    }

    Mesh::new(positions, colors, normals, indexes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::relative_eq;
    use js_sys::{Array, Function};
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;

    // テキスト（https://wgld.org/d/webgl/w021.html）に載っている
    // torus生成関数の単純な移植
    fn torus_origin_in_rust(row: usize, column: usize, irad: f32, orad: f32) -> Mesh {
        let mut pos = StepVec::<Vector3<f32>>::new();
        let mut nor = StepVec::<Vector3<f32>>::new();
        let mut col = StepVec::<Vector4<f32>>::new();
        let mut idx = StepVec::<Vector3<f32>>::new();

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
                pos.push(vec3(tx, ty, tz));
                nor.push(vec3(rx, ry, rz));
                let hsva = Hsva::new(360.0 / column as f32 * ii as f32, 1.0, 1.0, 1.0);
                let rgba = Srgba::from(hsva);
                col.push(vec4(
                    rgba.color.red,
                    rgba.color.green,
                    rgba.color.blue,
                    rgba.alpha,
                ));
            }
        }

        for i in 0..(row as i16) {
            for ii in 0..(column as i16) {
                let r = (column as i16 + 1) * i + ii;
                idx.push(vec3(r, r + column as i16 + 1, r + 1));
                idx.push(vec3(r + column as i16 + 1, r + column as i16 + 2, r + 1));
            }
        }

        Mesh {
            positions: pos,
            colors: col,
            indexes: idx,
            normals: nor,
        }
    }

    fn torus_origin_in_js(row: usize, column: usize, irad: f32, orad: f32) -> Vec<Vec<f32>> {
        let args = "row, column, irad, orad";
        let code = r#"
        const hsva = (h, s, v, a) => {
            if(s > 1 || v > 1 || a > 1){return;}
            var th = h % 360;
            var i = Math.floor(th / 60);
            var f = th / 60 - i;
            var m = v * (1 - s);
            var n = v * (1 - s * f);
            var k = v * (1 - s * (1 - f));
            var color = new Array();
            if(!s > 0 && !s < 0){
                color.push(v, v, v, a); 
            } else {
                var r = new Array(v, n, m, m, k, v);
                var g = new Array(k, v, v, n, m, m);
                var b = new Array(m, m, k, v, v, n);
                color.push(r[i], g[i], b[i], a);
            }
            return color;
        };

        var pos = new Array(), nor = new Array(),
            col = new Array(), idx = new Array();
        for(var i = 0; i <= row; i++){
            var r = Math.PI * 2 / row * i;
            var rr = Math.cos(r);
            var ry = Math.sin(r);
            for(var ii = 0; ii <= column; ii++){
                var tr = Math.PI * 2 / column * ii;
                var tx = (rr * irad + orad) * Math.cos(tr);
                var ty = ry * irad;
                var tz = (rr * irad + orad) * Math.sin(tr);
                var rx = rr * Math.cos(tr);
                var rz = rr * Math.sin(tr);
                pos.push(tx, ty, tz);
                nor.push(rx, ry, rz);
                var tc = hsva(360 / column * ii, 1, 1, 1);
                col.push(tc[0], tc[1], tc[2], tc[3]);
            }
        }
        for(i = 0; i < row; i++){
            for(ii = 0; ii < column; ii++){
                r = (column + 1) * i + ii;
                idx.push(r, r + column + 1, r + 1);
                idx.push(r + column + 1, r + column + 2, r + 1);
            }
        }
        return [pos, nor, col, idx];
        "#;

        let array: Array = Function::new_with_args(args, code)
            .apply(
                &JsValue::NULL,
                &[
                    JsValue::from_f64(row as f64),
                    JsValue::from_f64(column as f64),
                    JsValue::from_f64(irad as f64),
                    JsValue::from_f64(orad as f64),
                ]
                .iter()
                .collect(),
            )
            .unwrap()
            .into();
        array
            .iter()
            .map(|item| {
                Array::from(&item)
                    .iter()
                    .map(|i| i.as_f64().unwrap() as f32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    #[wasm_bindgen_test]
    fn test_torus() {
        let torus1 = torus(1.0, 64, 2.0, 64);
        let torus2 = torus_origin_in_rust(64, 64, 1.0, 2.0);
        assert_eq!(torus1, torus2);

        let torus1 = torus(1.0, 32, 2.0, 32);
        let torus2 = torus_origin_in_rust(32, 32, 1.0, 2.0);
        assert_eq!(torus1, torus2);

        let torus1 = torus(1.0, 32, 2.0, 32);
        let torus2 = torus_origin_in_js(32, 32, 1.0, 2.0);
        relative_eq!(torus1.positions.0.as_slice(), torus2[0].as_slice());
        relative_eq!(torus1.normals.0.as_slice(), torus2[1].as_slice());
        relative_eq!(torus1.colors.0.as_slice(), torus2[2].as_slice());
        assert_eq!(
            torus1.indexes.0,
            torus2[3].iter().map(|f| *f as i16).collect::<Vec<_>>()
        );
    }
}
