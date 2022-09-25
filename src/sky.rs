// sky.rs

use crate::bdg_color::ColorRgbF;
use crate::math::{Ray, array_interpolate};

#[derive(Debug, Copy, Clone)]
pub struct SkySphere {

}

impl SkySphere {
    pub fn shoot_ray(&self, r: Ray) -> ColorRgbF {

	let dirz = r.direction.z;

	let colors: [(f32, ColorRgbF); 5] = [
	    (-1.0, ColorRgbF::BLACK),
	    (-0.1, ColorRgbF::GREEN),
	    (0.0, ColorRgbF::WHITE),
	    (0.1, ColorRgbF::BLUE),
	    (1.0, ColorRgbF::BLACK)];
	   
	array_interpolate(dirz, &colors)
    }
}
