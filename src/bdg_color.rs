// bdg_color.rs
//
// provides RGB colors 0.0-255.0

use std::ops::{Add, Sub, Mul};


#[derive(Debug, Copy, Clone)]
pub struct ColorRgbF {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

#[allow(unused)]
impl ColorRgbF {
    pub const RED: ColorRgbF = ColorRgbF { r: 255.0, g: 0.0, b: 0.0 };
    pub const GREEN: ColorRgbF = ColorRgbF { r: 0.0, g: 255.0, b: 0.0 };
    pub const BLUE: ColorRgbF = ColorRgbF { r: 0.0, g: 0.0, b: 255.0 };
    pub const YELLOW: ColorRgbF = ColorRgbF { r: 255.0, g: 255.0, b: 0.0 };
    pub const CYAN: ColorRgbF = ColorRgbF { r: 0.0, g: 255.0, b: 255.0 };
    pub const MAGENTA: ColorRgbF = ColorRgbF { r: 255.0, g: 0.0, b: 255.0 };
    pub const BLACK: ColorRgbF = ColorRgbF { r: 0.0, g: 0.0, b: 0.0 };
    pub const GRAY_50: ColorRgbF = ColorRgbF { r: 128.0, g: 128.0, b: 128.0 };
    pub const WHITE: ColorRgbF = ColorRgbF { r: 255.0, g: 255.0, b: 255.0 };
}

impl Add for ColorRgbF {
    type Output = Self;
    fn add(self, o:Self) -> Self {
	Self {
	    r: self.r + o.r,
	    g: self.g + o.g,
	    b: self.b + o.b
	}
    }
}

impl Sub for ColorRgbF {
    type Output = Self;
    fn sub(self, o:Self) -> Self {
	Self {
	    r: self.r - o.r,
	    g: self.g - o.g,
	    b: self.b - o.b
	}
    }
}

impl Mul<f32> for ColorRgbF {
    type Output = Self;
    fn mul(self, s:f32) -> Self {
	Self {
	    r: self.r * s,
	    g: self.g * s,
	    b: self.b * s
	}
    }
}

impl Mul<ColorRgbF> for f32 {
    type Output = ColorRgbF;
    fn mul(self, c:ColorRgbF) -> ColorRgbF {
	ColorRgbF {
	    r: self * c.r,
	    g: self * c.g,
	    b: self * c.b
	}
    }
}



impl ColorRgbF {
    pub fn to_rgb8(self) -> ColorRgb8 {
	let r_8 = self.r as u8;
	let g_8 = self.g as u8;
	let b_8 = self.b as u8;

	ColorRgb8 {
	    r:r_8,
	    g:g_8,
	    b:b_8
	}
    }

    pub fn modulate(&self, o:&ColorRgbF) -> ColorRgbF {
	ColorRgbF {
	    r: (self.r / 255.0) * o.r,
	    g: (self.g / 255.0) * o.g,
	    b: (self.b / 255.0) * o.b,
	}
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> ColorRgbF {
	let chroma = v * s;
	let hue_val = (h / 60.0) % 2.0 - 1.0;
	let x = chroma * (1.0 - f32::abs(hue_val));
	let m = v - chroma;

	let rgb_p = if h < 60.0 {
	    (chroma, x, 0.0)
	} else if h < 120.0 {
	    (x, chroma, 0.0)
	} else if h < 180.0 {
	    (0.0, chroma, x)
	} else if h < 240.0 {
	    (0.0, x, chroma)
	} else if h < 300.0 {
	    (x, 0.0, chroma)
	} else {
	    (chroma, 0.0, x)
	};

	ColorRgbF {
	    r:(rgb_p.0 + m) * 255.0,
	    g:(rgb_p.1 + m) * 255.0,
	    b:(rgb_p.2 + m) * 255.0
	}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ColorRgb8 {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

