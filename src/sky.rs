// sky.rs


use crate::math::Ray;
use crate::bdg_color::ColorRGB_f;

pub struct SkySphere {

}

impl SkySphere {
    pub fn shoot_ray(&self, r: Ray) -> ColorRGB_f {
	if r.direction.z > 0.0 {
	    ColorRGB_f::CYAN
	} else {
	    ColorRGB_f::GREEN
	}
    }
}
