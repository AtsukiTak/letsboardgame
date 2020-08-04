#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        assert!(0.0 <= a && a <= 1.0);
        Color { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 1.0 }
    }

    pub fn black() -> Self {
        Color::rgb(0, 0, 0)
    }

    /// 0 ~ 1 の範囲に丸めた値を返す
    pub fn to_f32(self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a,
        )
    }
}
