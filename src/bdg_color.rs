// bdg_color.rs
//
// provides RGB colors 0.0-255.0

#[derive(Debug, Copy, Clone)]
pub struct ColorRGB_f {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl ColorRGB_f {
    pub const RED: ColorRGB_f = ColorRGB_f { r: 255.0, g: 0.0, b: 0.0 };
    pub const GREEN: ColorRGB_f = ColorRGB_f { r: 0.0, g: 255.0, b: 0.0 };
    pub const BLUE: ColorRGB_f = ColorRGB_f { r: 0.0, g: 0.0, b: 255.0 };
    pub const YELLOW: ColorRGB_f = ColorRGB_f { r: 255.0, g: 255.0, b: 0.0 };
    pub const CYAN: ColorRGB_f = ColorRGB_f { r: 0.0, g: 255.0, b: 255.0 };
    pub const MAGENTA: ColorRGB_f = ColorRGB_f { r: 255.0, g: 0.0, b: 255.0 };
    pub const BLACK: ColorRGB_f = ColorRGB_f { r: 0.0, g: 0.0, b: 0.0 };
    pub const GRAY_50: ColorRGB_f = ColorRGB_f { r: 128.0, g: 128.0, b: 128.0 };
    pub const WHITE: ColorRGB_f = ColorRGB_f { r: 255.0, g: 255.0, b: 255.0 };

    pub fn to_rgb8(self) -> ColorRGB_8 {
	let r_8 = self.r as u8;
	let g_8 = self.g as u8;
	let b_8 = self.b as u8;

	ColorRGB_8 {
	    r:r_8,
	    g:g_8,
	    b:b_8
	}
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> ColorRGB_f {
	let chroma = v * s;
	let hue_val = (h / 60.0) % 2.0 - 1.0;
	let x = chroma * (1.0 - f32::abs(hue_val));
	let m = v - chroma;

	let mut rgb_p = (0.0, 0.0, 0.0);

	if h < 60.0 {
	    rgb_p = (chroma, x, 0.0);
	} else if h < 120.0 {
	    rgb_p = (x, chroma, 0.0);
	} else if h < 180.0 {
	    rgb_p = (0.0, chroma, x);
	} else if h < 240.0 {
	    rgb_p = (0.0, x, chroma);
	} else if h < 300.0 {
	    rgb_p = (x, 0.0, chroma);
	} else {
	    rgb_p = (chroma, 0.0, x);
	}

	ColorRGB_f {
	    r:(rgb_p.0 + m) * 255.0,
	    g:(rgb_p.1 + m) * 255.0,
	    b:(rgb_p.2 + m) * 255.0
	}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ColorRGB_8 {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

