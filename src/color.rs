// color.rs
//
// provides RGB colors 0.0-255.0

pub struct ColorRGB_f {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl ColorRGB_f {
    const RED: ColorRGB_f = ColorRGB_f { r: 255.0, g: 0.0, b: 0.0 };
    const GREEN: ColorRGB_f = ColorRGB_f { r: 0.0, g: 255.0, b: 0.0 };
    const BLUE: ColorRGB_f = ColorRGB_f { r: 0.0, g: 0.0, b: 255.0 };
    const YELLOW: ColorRGB_f = ColorRGB_f { r: 255.0, g: 255.0, b: 0.0 };
    const CYAN: ColorRGB_f = ColorRGB_f { r: 0.0, g: 255.0, b: 255.0 };
    const MAGENTA: ColorRGB_f = ColorRGB_f { r: 255.0, g: 0.0, b: 255.0 };
    const BLACK: ColorRGB_f = ColorRGB_f { r: 0.0, g: 0.0, b: 0.0 };
    const GRAY_50: ColorRGB_f = ColorRGB_f { r: 128.0, g: 128.0, b: 128.0 };
    const WHITE: ColorRGB_f = ColorRGB_f { r: 255.0, g: 255.0, b: 255.0 };
}
