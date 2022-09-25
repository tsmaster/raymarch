// sky.rs

use crate::math::Ray;
use crate::bdg_color::ColorRgbF;

#[derive(Debug, Copy, Clone)]
pub struct SkySphere {

}

impl SkySphere {
    pub fn shoot_ray(&self, r: Ray) -> ColorRgbF {
	if r.direction.z > 0.0 {
	    ColorRgbF::CYAN
	} else {
	    ColorRgbF::GREEN
	}
    }
}
